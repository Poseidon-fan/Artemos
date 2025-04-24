mod heap_allocator;
mod address;
mod page_table;
mod frame_allocator;
mod memory_set;
pub use address::{PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use frame_allocator::{FrameTracker, frame_alloc, frame_dealloc};
pub use memory_set::remap_test;
pub use memory_set::{KERNEL_SPACE, MapPermission, MemorySet, kernel_token};
pub use page_table::{
    PageTable, PageTableEntry, UserBuffer, UserBufferIterator, translated_byte_buffer,
    translated_ref, translated_refmut, translated_str,
};
pub fn heap_test() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}

pub fn frame_allocator_test() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    frame_allocator::frame_allocator_test();
}

pub fn init() {
    // 使能动态内存分配
    heap_allocator::init_heap();
    // 使能物理页帧管理
    frame_allocator::init_frame_allocator();
    // 这里的 KERNEL_SPACE 就是内核地址空间的全局实例
    // 并让 CPU 开启分页模式
    KERNEL_SPACE.exclusive_access().activate();
}