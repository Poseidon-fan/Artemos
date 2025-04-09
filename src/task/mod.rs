use alloc::sync::Arc;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use sbi::shutdown;
use crate::loader::{get_app_data, get_num_app};
use crate::sync::UPSafeCell;
use crate::task::context::TaskContext;
use crate::task::switch::__switch;
use crate::task::task::{TaskControlBlock, TaskStatus};
use crate::trap::TrapContext;

mod task;
mod context;
mod switch;
mod pid;
mod processor;
mod manager;

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        println!("init TASK_MANAGER");
        // 获取应用数量
        let num_app = get_num_app();
        println!("num_app = {}", num_app);
        let mut tasks: Vec<TaskControlBlock> = Vec::new();
        // 获取每个任务的任务控制块，并推入 tasks 中
        for i in 0..num_app {
            tasks.push(TaskControlBlock::new(
                get_app_data(i),
                i,
            ));
        }
        TaskManager {
            num_app,
            inner: unsafe{
               UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
               })
            }
        }
    };
}

use crate::loader::get_app_data_by_name;
use manager::add_task;
use crate::task::processor::{schedule, take_current_task};

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap()
    ));
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

impl TaskManager {
    /// Run the first task in task list.
    ///
    /// Generally, the first task in task list is an idle task (we call it zero process later).
    /// But in ch3, we load apps statically, so the first task is a real app.
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

    /// Change the status of current `Running` task into `Ready`.
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    /// Change the status of current `Running` task into `Exited`.
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    /// Find next task to run and return app id.
    ///
    /// In this case, we only return the first `Ready` task in task list.
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    /// Switch current `Running` task to the task we have found,
    /// or there is no `Ready` task and we can exit with all applications completed
    fn run_next_task(&self) {
        // 首先调用 find_next_task 寻找一个状态为 Ready 的任务
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            // 如果不手动释放借用的话，等 __switch 返回时才会释放，这期间我们都不能读写 TaskManagerInner 了
            drop(inner);
            // before this, we should drop local variables that must be dropped manually
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // go back to user mode
        } else {
            println!("All applications completed!");
            shutdown(false);
        }
    }

    // 获得当前正在执行的应用的地址空间的 token
    fn get_current_token(&self) -> usize {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].get_user_token()
    }

    // 获得当前正在执行的应用的地址空间中的 Trap 上下文
    fn get_current_trap_cx(&self) -> &mut TrapContext {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].get_trap_cx()
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

/// rust next task
fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/// suspend current task
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

/// exit current task
fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

// 这个时候要根据 exit_code 进行相应的处理
pub fn exit_current_and_run_next(exit_code: i32) {
    // 获取当前的任务
    let task = take_current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    // 当当前任务的状态设置为 Zombie
    inner.task_status = TaskStatus::Zombie;
    // 记录 exit_code 给父亲看
    inner.exit_code = exit_code;

    // 将当前进程的所有子进程挂在初始进程 initproc 下面
    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ stop exclusively accessing parent PCB

    // 清空自己的孩子
    inner.children.clear();
    // 释放自己占有的地址空间
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** stop exclusively accessing current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    // 调度到下一个，现在不关心上文，只关心下文
    schedule(&mut _unused as *mut _);
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}

pub fn suspend_current_and_run_next() {
    // 获取当前正在执行的任务
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- stop exclusively accessing current PCB

    // 将这个任务重新放回就绪队列
    add_task(task);
    // 使用 __switch 进行任务上下文的切换
    schedule(task_cx_ptr);
}