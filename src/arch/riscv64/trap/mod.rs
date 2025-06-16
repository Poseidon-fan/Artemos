mod context;

use core::arch::global_asm;

use riscv::register::{mtvec::TrapMode, stvec};

global_asm!(include_str!("trap.asm"));

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

#[no_mangle]
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
