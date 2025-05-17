use core::{
    arch::global_asm,
    sync::atomic::{AtomicBool, Ordering},
};

use fdt::Fdt;
use log::info;

use crate::{
    arch::{
        sbi::{self},
        system,
    },
    logging,
};

global_asm!(include_str!("entry.asm"));

static FIRST_HART: AtomicBool = AtomicBool::new(true);

#[unsafe(no_mangle)]
pub fn kernel_main(hartid: usize, device_tree_paddr: usize) -> ! {
    if FIRST_HART
        .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        logging::init();
        info!("hart: {} is starting", hartid);
        // get hart count from device tree
        let fdt = unsafe { Fdt::from_ptr(device_tree_paddr as *const u8).unwrap() };
        let mut hart_count = 0;
        for node in fdt.find_all_nodes("/cpus/cpu") {
            if let Some(_reg) = node.property("reg") {
                hart_count += 1;
            }
        }
        info!("hart_count: {}", hart_count);
        (0..hart_count).filter(|&i| i != hartid).for_each(|i| {
            sbi::start_hart(i, 0x80200000, 0);
        });
        loop {
            unsafe {
                system::halt();
            }
        }
    } else {
        others_main(hartid)
    }
}

fn others_main(hart_id: usize) -> ! {
    info!("hart: {} is starting", hart_id);
    loop {
        unsafe {
            system::halt();
        }
    }
}
