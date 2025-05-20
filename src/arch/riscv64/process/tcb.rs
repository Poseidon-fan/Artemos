use alloc::sync::Weak;

use super::pcb::ProcessControlBlock;

pub struct ThreadControlBlock {
    tid: Tid,
    process: Weak<ProcessControlBlock>,
    status: ThreadStatus,
    ustack_top: usize,
}

#[derive(Clone, Copy, Debug)]
struct Tid(usize);

impl ThreadControlBlock {
    fn get_tid(&self) -> Tid {
        self.tid
    }
}

enum ThreadStatus {
    Running,
}
