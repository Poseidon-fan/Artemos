use core::{
    arch::{asm, global_asm},
    sync::atomic::{AtomicBool, Ordering},
};

use fdt::Fdt;
use log::info;

use crate::{
    arch::{cpu, sbi, system},
    logging,
};

global_asm!(include_str!("entry.asm"));

static FIRST_HART: AtomicBool = AtomicBool::new(true);


const TMP: usize = 0xffff_ffc0_0000_0000;
#[unsafe(no_mangle)]
pub fn fake_main(hart_id: usize, device_tree_paddr: usize) {
    unsafe {
        asm!("add sp, sp, {}", in(reg) TMP);
        asm!("la t0, kernel_main");
        asm!("add t0, t0, {}", in(reg) TMP);
        asm!("mv a0, {}", in(reg) hart_id);
        asm!("mv a1, {}", in(reg) device_tree_paddr);
        asm!("jalr zero, 0(t0)");
    }
}

#[unsafe(no_mangle)]
pub fn kernel_main(hart_id: usize, device_tree_paddr: usize) -> ! {
    if FIRST_HART
        .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        others_main(hart_id)
    }

    // main hart initialization
    clear_bss();
    logging::init();
    cpu::init_local_cpu_context(hart_id);
    info!("hart: {} is starting", hart_id);

    // trigger other harts to start
    trigger_other_harts(hart_id, device_tree_paddr);
    loop {
        system::halt();
    }
}

fn others_main(hart_id: usize) -> ! {
    cpu::init_local_cpu_context(hart_id);
    info!("hart: {} is starting", cpu::hart_id());
    loop {
        system::halt();
    }
}

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

fn trigger_other_harts(hart_id: usize, device_tree_paddr: usize) {
    // get hart count from device tree
    let fdt = unsafe { Fdt::from_ptr((device_tree_paddr + TMP) as *const u8).unwrap() };
    let mut hart_count = 0;
    for node in fdt.find_all_nodes("/cpus/cpu") {
        if let Some(_reg) = node.property("reg") {
            hart_count += 1;
        }
    }
    info!("hart_count: {}", hart_count);
    (0..hart_count).filter(|&i| i != hart_id).for_each(|i| {
        sbi::start_hart(i, 0x80200000, 0);
    });
}
