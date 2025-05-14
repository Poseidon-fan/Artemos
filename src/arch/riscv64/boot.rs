use core::{
    arch::global_asm,
    sync::atomic::{AtomicBool, Ordering},
};

use log::info;

use crate::{
    arch::sbi::{self},
    logging,
};

global_asm!(include_str!("entry.asm"));

static FIRST_HART: AtomicBool = AtomicBool::new(true);

#[unsafe(no_mangle)]
pub fn kernel_main(hartid: usize, _device_tree_paddr: usize) -> ! {
    if FIRST_HART
        .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        logging::init();
        info!("hart: {} is starting", hartid);
        for i in 0..4 {
            if i != hartid {
                sbi::start_hart(i, 0x80200000, 0);
            }
        }
        loop {}
    } else {
        others_main(hartid)
    }
}

fn others_main(hart_id: usize) -> ! {
    info!("hart: {} is starting", hart_id);
    loop {}
}
