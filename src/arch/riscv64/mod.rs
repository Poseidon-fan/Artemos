pub mod console;
mod sbi;

use core::arch::global_asm;

use log::info;

use crate::logging;

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    logging::init();
    info!("kernel start");
    panic!("kernel shutdown")
}
