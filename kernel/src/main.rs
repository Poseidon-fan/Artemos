#![no_std]
#![no_main]

mod panic;
mod sbi;
mod console;

use core::arch::global_asm;

global_asm!(include_str!("entry.S"));

pub fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("manually panic")
}

