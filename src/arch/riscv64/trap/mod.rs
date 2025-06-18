pub mod context;

use core::arch::global_asm;

use riscv::register::{mtvec::TrapMode, scause, sepc, sie, stval, stvec};

use crate::arch::{
    config::CLOCK_FREQ,
    mm::address::{VirtAddr, VirtPageNum},
};

global_asm!(include_str!("trap.asm"));

unsafe extern "C" {
    fn __trap_from_user();
}
// when we call this function, we are in kernel mode
// we will set the trap entry to kernel_trap
pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(__trap_from_user as usize, TrapMode::Direct);
    }
}

/// enable timer interrupt in sie CSR
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[unsafe(no_mangle)]
pub fn trap_from_kernel() -> ! {
    let stval = stval::read();
    let sepc = sepc::read();
    let stval_vpn = VirtPageNum::from(VirtAddr::from(stval));
    let sepc_vpn = VirtPageNum::from(VirtAddr::from(sepc));
    panic!(
        "stval = {:#x}(vpn {}), sepc = {:#x}(vpn{}),
        a trap {:?} from kernel!",
        stval,
        stval_vpn.0,
        sepc,
        sepc_vpn.0,
        scause::read().cause()
    );
}
