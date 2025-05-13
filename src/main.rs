#![no_std]
#![no_main]

mod panic;

#[cfg(target_arch = "riscv64")]
#[path = "arch/riscv64/mod.rs"]
mod arch;

#[cfg(target_arch = "loongarch64")]
#[path = "arch/loongarch64/mod.rs"]
mod arch;
