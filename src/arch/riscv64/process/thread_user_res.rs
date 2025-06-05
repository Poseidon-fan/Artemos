use alloc::sync::Arc;

use lazy_static::*;
use spin::Mutex;

use crate::arch::{mm::address::PhysPageNum, process::pcb::ProcessControlBlock, utils::QueueAllocator};

pub struct ThreadUserRes {
    tid: Tid,
    pub user_stack_base: usize,
    trap_cx_ppn: PhysPageNum,
}

impl ThreadUserRes {
    // todo: 用户程序的地址空间是如何设计的
    pub fn new(process: Arc<ProcessControlBlock>, user_stack_base: usize) -> Self {
        // alloc tid
        let tid = process.inner_exclusive_access().tid_allocator.lock().alloc();
        ThreadUserRes {
            tid: Tid(tid),
            user_stack_base,
            trap_cx_ppn: PhysPageNum(1),
        }
    }
}

#[derive(Clone, Debug)]
struct Tid(usize);

lazy_static! {
    static ref TID_ALLOCATOR: Mutex<QueueAllocator> = Mutex::new(QueueAllocator::new());
}

// RAII
fn tid_alloc() -> Tid {
    Tid(TID_ALLOCATOR.lock().alloc())
}

impl Drop for Tid {
    fn drop(&mut self) {
        TID_ALLOCATOR.lock().dealloc(self.0);
    }
}
