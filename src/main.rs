#![no_main]
// 不再使用 Rust 标准库 std，转而使用核心库 core
// 核心库 core 可以直接在裸机上使用，但 std 不能，需要 OS 的支持
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::arch::global_asm;
use log::info;

#[path = "boards/qemu.rs"]
mod board;

mod panic;
#[macro_use]
mod logging;
#[macro_use]
extern crate bitflags;
mod sync;
mod syscall;
mod loader;
mod config;
mod task;
mod timer;
mod trap;
mod mm;

// include_str! 宏的作用是将文件内容作为字符串常量嵌入到程序
// global_asm! 宏的作用是将汇编代码嵌入到程序中
global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    clear_bss();
    logging::init();
    info!("[kernel] Hello, world!");
    mm::init();
    info!("[kernel] back to world!");
    // 检查内核地址空间的多级页表是否被正确设置
    mm::remap_test();
    // trap::init();
    // trap::enable_interrupt();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
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