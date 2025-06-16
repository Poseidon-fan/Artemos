use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};
use core::cell::RefMut;

use lazy_static::lazy_static;
use spin::Mutex;

use super::tcb::ThreadControlBlock;
use crate::arch::{
    mm::memory_set::{self, MemorySet},
    utils::QueueAllocator,
};


pub struct ProcessControlBlock {
    pid: Pid,
    inner: Mutex<ProcessControlBlockInner>,
}

pub struct ProcessControlBlockInner {
    parent: Option<Weak<ProcessControlBlock>>,
    children: Vec<Arc<ProcessControlBlock>>,
    status: ProcessStatus,
    exit_code: i32,
    threads: Vec<Option<Arc<ThreadControlBlock>>>,
    pub tid_allocator: Mutex<QueueAllocator>,
    memory: MemorySet,
    // fd_table: Vec<Option<Arc<File>>>,
    // cwd: Arc<Dir>,
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
    Normal,
    Zombie,
    Exited,
}

impl ProcessControlBlock {
    // only initproc can be created by hand
    // other process should be created by fork or exec
    pub fn init_initproc(elf_data: &[u8]) -> Arc<Self> {
        let (memory_set, entry_point, user_heap_bottom) = MemorySet::from_elf(elf_data);
        let pcb = Arc::new(Self {
            pid: pid_alloc(),
            inner: Mutex::new(ProcessControlBlockInner {
                parent: None,
                children: Vec::new(),
                status: ProcessStatus::Normal,
                exit_code: 0,
                threads: Vec::new(),
                tid_allocator: Mutex::new(QueueAllocator::new()),
                memory: memory_set,
            }),
        });
    }

    pub fn inner_exclusive_access(&self) -> spin::MutexGuard<'_, ProcessControlBlockInner> {
        self.inner.lock()
    }

    fn add_thread(thread: ThreadControlBlock) {
        // todo: add thread to current PCB
    }

    pub fn exec(self: &Arc<Self>, elf_data: &[u8]) {}
}

impl ProcessControlBlockInner {
    pub fn get_thread(&self, tid: usize) -> Arc<ThreadControlBlock> {
        self.threads[tid].as_ref().unwrap().clone()
    }
}
