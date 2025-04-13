use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::lazy_static;
use crate::sync::UPSafeCell;
use crate::task::task::TaskControlBlock;

pub struct TaskManager {
    /* 关于这里为什么要用Arc，而不是Box或者裸引用
    因为除了TaskManager来管理TaskControlBlock之外，还有真正**使用**它的东西，
    比如其父进程、其子进程等 */
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self { ready_queue: VecDeque::new(), }
    }
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> = unsafe {
        UPSafeCell::new(TaskManager::new())
    };
}

// 提供给外界的包装
pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}