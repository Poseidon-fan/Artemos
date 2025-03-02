#![no_main]
#![no_std]

use core::arch::global_asm;
use log::{info, warn};

mod panic;
mod logging;

global_asm!(include_str!("entry.S"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    clear_bss();
    logging::init();
    info!("Hello, world!");
    warn!("This is a warning");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}