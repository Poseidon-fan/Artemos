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
    frame_allocator_test();
    info!(
        "frame allocator init successfully, start {:#x}, end {:#x}",
        ekernel as usize, MEMORY_END
    );
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR.lock().alloc().map(FrameTracker::new)
}

fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.lock().dealloc(ppn);
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

pub struct FrameTracker {
    pub ppn: PhysPageNum,
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

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        FrameTracker { ppn }
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

#[allow(unused)]
/// a simple test for frame allocator
pub fn frame_allocator_test() {
    use crate::println;
    info!("frame_allocator_test start...");
    let mut v: Vec<FrameTracker> = Vec::new();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("alloca frame: {}", frame.ppn.0);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("alloca frame: {}", frame.ppn.0);
        v.push(frame);
    }
    drop(v);
    info!("frame_allocator_test passed!");
}
