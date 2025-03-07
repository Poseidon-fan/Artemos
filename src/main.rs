#![no_main]
// 不再使用 Rust 标准库 std，转而使用核心库 core
// 核心库 core 可以直接在裸机上使用，但 std 不能，需要 OS 的支持
#![no_std]

use core::arch::global_asm;

mod panic;
#[macro_use]
mod logging;
mod batch;
mod sync;
mod trap;
mod syscall;

// include_str! 宏的作用是将文件内容作为字符串常量嵌入到程序
// global_asm! 宏的作用是将汇编代码嵌入到程序中
global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    batch::print_stack_location();
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