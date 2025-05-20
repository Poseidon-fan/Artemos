use core::arch::asm;

use super::sbi;

/// shutdown the system
#[inline(always)]
pub fn shutdown() -> ! {
    sbi::shutdown()
}

/// Halt instruction
#[inline(always)]
pub fn halt() {
    unsafe { asm!("wfi", options(nomem, nostack)) }
}
