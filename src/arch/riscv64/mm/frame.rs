use alloc::vec::Vec;

use lazy_static::lazy_static;
use log::{info, warn};
use spin::Mutex;

use super::address::PhysPageNum;
use crate::arch::{
    board::qemu::MEMORY_END,
    mm::address::{VirtAddr, kva2pa},
};

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<StackFrameAllocator> = Mutex::new(StackFrameAllocator::new());
}

pub fn init_frame_allocator() {
    unsafe extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR.lock().init(
        kva2pa(VirtAddr(ekernel as usize)).ceil(),
        kva2pa(VirtAddr(MEMORY_END)).floor(),
    );
    info!(
        "frame allocator init successfully, start {:#x}, end {:#x}",
        ekernel as usize, MEMORY_END
    );
}

trait FrameAllocator {
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else if self.current == self.end {
            warn!("no more frames to alloc");
            None
        } else {
            self.current += 1;
            Some((self.current - 1).into())
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        // validity check
        if ppn >= self.current || self.recycled.iter().any(|&v| v == ppn) {
            panic!("Frame ppn={ppn} has not been allocated!");
        }
        // recycle
        self.recycled.push(ppn);
    }
}

impl StackFrameAllocator {
    const fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    pub fn init(&mut self, start: PhysPageNum, end: PhysPageNum) {
        self.current = start.0;
        self.end = end.0;
    }
}
