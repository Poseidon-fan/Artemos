use crate::mm::heap_allocator::init_heap;

mod heap_allocator;

pub fn heap_test() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}