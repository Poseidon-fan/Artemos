use core::{
    arch::{asm, global_asm},
    sync::atomic::{AtomicBool, Ordering},
};

use fdt::Fdt;
use log::info;

use crate::{
    arch::{config::KERNEL_ADDR_OFFSET, cpu, mm, process, sbi, system},
    loader, logging,
};

global_asm!(include_str!("entry.asm"));

static FIRST_HART: AtomicBool = AtomicBool::new(true);

/// Rust entry, called from entry.asm.
/// Mainly used to convert the abs address to virtual address, since we've already configured the
/// paging table, and jump to kernel_main.
/// Things have to convert:
/// - sp: point to kernel boot stack
/// - kernel_main: jump to kernel_main by virtual address
/// - device_tree_paddr: convert to virtual address and pass to kernel_main
#[unsafe(no_mangle)]
pub fn rust_entry(hart_id: usize, device_tree_paddr: usize) {
    unsafe {
        // setup sp
        asm!("add sp, sp, {}", in(reg) KERNEL_ADDR_OFFSET);

        // calculate kernel_main virtual address
        asm!("la t0, kernel_main");
        asm!("add t0, t0, {}", in(reg) KERNEL_ADDR_OFFSET);
        // save hart_id and pass as arg 0
        asm!("mv a0, {}", in(reg) hart_id);
        // convert device_tree_paddr to virtual address and pass as arg 1
        asm!("mv a1, {}", in(reg) device_tree_paddr | KERNEL_ADDR_OFFSET);
        // jump to kernel_main
        asm!("jalr zero, 0(t0)");
    }
}

/// Core setup code for each hart.
/// if this is not the first hart, call others_main to start the hart,
/// otherwise, initialize the hart and trigger other harts to start.
#[unsafe(no_mangle)]
pub fn kernel_main(hart_id: usize, device_tree_vaddr: usize) -> ! {
    if FIRST_HART
        .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        others_main(hart_id)
    }

    // main hart initialization
    clear_bss();
    logging::init();
    info!("hart: {} is starting", hart_id);

    // store local cpu context
    cpu::init_local_cpu_context(hart_id);

    mm::init();
    loader::init();
    process::add_initproc();

    // trigger other harts to start
    trigger_other_harts(hart_id, device_tree_vaddr);
    loop {
        system::halt();
    }
}

/// Other harts entry, called from kernel_main.
fn others_main(hart_id: usize) -> ! {
    cpu::init_local_cpu_context(hart_id);
    mm::activate_kernel_space();
    info!("hart: {} is starting", cpu::hart_id());
    loop {
        system::halt();
    }
}

/// Clear kernel BSS section mannually.
fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

fn trigger_other_harts(hart_id: usize, device_tree_vaddr: usize) {
    // get hart count from device tree
    let fdt = unsafe { Fdt::from_ptr(device_tree_vaddr as *const u8).unwrap() };
    let mut hart_count = 0;
    for node in fdt.find_all_nodes("/cpus/cpu") {
        if let Some(_reg) = node.property("reg") {
            hart_count += 1;
        }
    }
    info!("hart_count: {}", hart_count);
    (0..hart_count).filter(|&i| i != hart_id).for_each(|i| {
        // pass paddr here, because other harts don't start paging yet
        sbi::start_hart(i, 0x80200000, 0);
    });
}
