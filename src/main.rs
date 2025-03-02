#![no_main]
#![no_std]

use core::arch::global_asm;

mod panic;
#[macro_use]
mod logging;
mod batch;
mod sync;
mod trap;
mod syscall;

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    clear_bss();
    logging::init();
    trap::init();
    batch::init();
    batch::run_next_app();
}

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}