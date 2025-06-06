use alloc::sync::{Arc, Weak};
use core::cell::RefMut;

use spin::Mutex;

use super::pcb::ProcessControlBlock;
use crate::arch::process::{context::ThreadContext, thread_user_res::ThreadUserRes};

// todo: kstack
pub struct ThreadControlBlock {
    process: Weak<ProcessControlBlock>,
    inner: Mutex<ThreadControlBlockInner>,
}

// todo: how to design trap context
pub struct ThreadControlBlockInner {
    res: Option<ThreadUserRes>,
    thread_context: ThreadContext,
    thread_status: ThreadStatus,
    exit_code: i32,
}

// impl ThreadControlBlock {
//     fn get_tid(&self) -> Tid {
//         self.tid
//     }
// }

impl ThreadControlBlock {
    pub fn inner_exclusive_access(&self) -> spin::MutexGuard<'_, ThreadControlBlockInner> {
        self.inner.lock()
    }

    pub fn new(process: Arc<ProcessControlBlock>, user_stack_base: usize) -> Self {
        let res = ThreadUserRes::new(process.clone(), user_stack_base);
        Self {
            process: Arc::downgrade(&process),
            inner: Mutex::new(ThreadControlBlockInner {
                res: Some(res),
                thread_context: ThreadContext::zero_init(),
                thread_status: ThreadStatus::Ready,
                exit_code: 0,
            }),
        }
    }
}

impl ThreadControlBlockInner {
    fn get_status(&self) -> ThreadStatus {
        self.thread_status
    }

    pub fn get_user_stack_base(&self) -> usize {
        self.res.as_ref().unwrap().user_stack_base
    }
}

#[derive(Copy, Clone, PartialEq)]
enum ThreadStatus {
    Ready,
    Running,
    Blocked,
}
