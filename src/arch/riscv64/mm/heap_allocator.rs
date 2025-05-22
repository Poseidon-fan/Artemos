use core::ptr::addr_of_mut;

use buddy_system_allocator::LockedHeap;
use log::info;

use crate::arch::config::KERNEL_HEAP_SIZE;


#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<16> = LockedHeap::empty();

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("heap allocation error: {:?}", layout);
}


pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(addr_of_mut!(HEAP_SPACE) as usize, KERNEL_HEAP_SIZE);
    }
    heap_test();
}

fn heap_test() {
    use alloc::vec;
    let t = vec![1, 2, 3, 4, 5];
    for i in t {
        info!("this is heap test: {}", i);
    }
}
