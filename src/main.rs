#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::arch::global_asm;

extern crate alloc;

mod loader;
mod logging;
mod panic;

global_asm!(include_str!("link_app.asm"));

#[cfg(target_arch = "riscv64")]
#[path = "arch/riscv64/mod.rs"]
mod arch;

#[cfg(target_arch = "loongarch64")]
#[path = "arch/loongarch64/mod.rs"]
mod arch;
