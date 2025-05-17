use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};

use lazy_static::lazy_static;
use spin::Mutex;

use super::tcb::ThreadControlBlock;
use crate::arch::utils::QueueAllocator;

struct ProcessControlBlock {
    pid: Pid,
    parent: Option<Weak<Mutex<ProcessControlBlock>>>,
    children: Vec<Arc<ProcessControlBlock>>,
    status: ProcessStatus,
    exit_code: i32,
    threads: Vec<Option<Arc<ThreadControlBlock>>>,
    tid_allocator: Mutex<QueueAllocator>,
    memory: MemorySet,
    fd_table: Vec<Option<Arc<File>>>,
    cwd: Arc<Dir>,
}

struct Pid(usize);

lazy_static! {
    static ref PID_ALLOCATOR: Mutex<QueueAllocator> = Mutex::new(QueueAllocator::new());
}

fn pid_alloc() -> Pid {
    Pid(PID_ALLOCATOR.lock().alloc())
}

impl Drop for Pid {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().dealloc(self.0);
    }
}

enum ProcessStatus {
    Running,
    Zombie,
    Exited,
}

impl ProcessControlBlock {}
