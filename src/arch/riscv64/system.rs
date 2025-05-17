use core::arch::asm;

use super::sbi;

/// shutdown the system
pub fn shutdown() -> ! {
    sbi::shutdown()
}

/// Halt instruction
#[inline(always)]
pub unsafe fn halt() {
    unsafe { asm!("wfi", options(nomem, nostack)) }
}
