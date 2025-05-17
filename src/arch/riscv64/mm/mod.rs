mod heap_allocator;

pub fn mm_init() {
    // Initialize heap space
    heap_allocator::init();
}
