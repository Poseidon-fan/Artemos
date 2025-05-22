mod address;
mod frame;
mod heap_allocator;
mod paging;

pub fn init() {
    heap_allocator::init_heap();
    frame::init_frame_allocator();
}
