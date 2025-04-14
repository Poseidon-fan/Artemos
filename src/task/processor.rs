use super::__switch;
use super::TaskStatus;
use super::{TaskContext, TaskControlBlock};
use crate::sync::UPSafeCell;
use crate::task::manager::fetch_task;
use crate::trap::TrapContext;
use alloc::sync::Arc;
use lazy_static::*;

pub struct Processor {
    /* 关于这里为什么要用Arc，而不是Box或者裸引用
    因为除了TaskManager来管理TaskControlBlock之外，还有真正**使用**它的东西，
    比如其父进程、其子进程等 */
    current: Option<Arc<TaskControlBlock>>,
    /// 空闲任务（idle task）是操作系统中 CPU 没有其他任务可运行时的“占位”状态。
    /// 它不是真正的用户任务，而是内核的特殊状态，用于避免 CPU 空转。
    /// 
    /// 可以举例追溯 sys_yield 的实现，发现它调用 suspend_current_and_run_next，后者调用 schedule。
    /// 即：进程的内核栈主动请求了将自己换出，换成新的进程。
    /// 
    /// 因此，使用一个空上下文作为占位，内核的自己的调度流就不需要关心现在是谁在运行，只要换成下一个就绪任务就行，完成了解耦。
    /// 
    /// 其值：始终为全0（zero_init）
    idle_task_cx: TaskContext,
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}

//? 这些个pub是不是也不需要
impl Processor {
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(), // 全0
        }
    }

    // 取出当前正在执行的任务
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    // 返回当前正在执行的任务的一个拷贝
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }

    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().current()
}

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    let token = task.inner_exclusive_access().get_user_token();
    token
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}

pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.exclusive_access();
        // 从就绪队列中取出队首
        if let Some(task) = fetch_task() {
            // 获取 switch 的第一个参数
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            // access coming task TCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            // 获取 switch 的第二个参数
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            // stop exclusively accessing coming task TCB manually
            drop(task_inner);
            processor.current = Some(task);
            // stop exclusively accessing processor manually
            drop(processor);
            unsafe {
                __switch(
                    idle_task_cx_ptr,
                    next_task_cx_ptr,
                );
            }
        }
    }
}

// 开启新一轮任务调用
pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.exclusive_access();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(
            switched_task_cx_ptr,
            idle_task_cx_ptr,
        );
    }
}