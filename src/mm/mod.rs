use crate::mm::heap_allocator::init_heap;

mod heap_allocator;
mod address;
mod page_table;
mod frame_allocator;

pub fn heap_test() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}

pub fn frame_allocator_test() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    frame_allocator::frame_allocator_test();
}