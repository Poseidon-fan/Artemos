pub mod address;
mod frame;
mod heap_allocator;
mod map_area;
pub mod memory_set;
mod paging;

pub use memory_set::activate_kernel_space;

pub fn init() {
    heap_allocator::init_heap();
    frame::init_frame_allocator();
    memory_set::activate_kernel_space();
}
