//! The main module and entrypoint
//!
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//!
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`task`]: Task management
//! - [`syscall`]: System call handling and implementation
//! - [`mm`]: Address map using SV39
//! - [`sync`]: Wrap a static data structure inside it so that we are able to access it without any `unsafe`.
//! - [`fs`]: Separate user from file system with some structures
//!
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source code for
//! details.)
//!
//! We then call [`task::run_tasks()`] and for the first time go to
//! userspace.

// #![deny(missing_docs)]
// #![deny(warnings)]
#![allow(unused_imports)]
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

use log::*;

#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod config;
mod drivers;
pub mod fs;
pub mod lang_items;
mod logging;
pub mod mm;
pub mod sync;
pub mod syscall;
pub mod task;
pub mod timer;
pub mod trap;

use core::arch::global_asm;

global_asm!(include_str!("entry.S"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    clear_bss();
    logging::init();
    info!("[kernel] Hello, world!");
    mm::init();
    info!("[kernel] back to world!");
    // 检查内核地址空间的多级页表是否被正确设置
    mm::remap_test();
    println!("after initproc!");
    trap::init();
    trap::enable_timer_interrupt();
    fs::list_apps();
    timer::set_next_trigger();
    task::add_initproc();
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
