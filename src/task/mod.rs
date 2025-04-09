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

pub use processor::{
    Processor, current_task, run_tasks, current_user_token, current_trap_cx
};
pub use manager::add_task;


use crate::loader::get_app_data_by_name;
use crate::task::processor::{schedule, take_current_task};

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap()
    ));
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
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