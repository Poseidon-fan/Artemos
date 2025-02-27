#![no_main]
#![no_std]

use core::arch::global_asm;

mod panic;
mod console;

global_asm!(include_str!("entry.S"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
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