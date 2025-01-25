#![no_std]
#![no_main]

mod panic;

use core::arch::global_asm;

global_asm!(include_str!("entry.S"));
