use core::arch::asm;

const SBI_CONSOLE_PUTCHAR: usize = 1;

/// general sbi call
#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x16") fid,  // generally set to 0
            in("x17") eid,
        );
    }
    ret
}

/// put a character to console, sbi interface
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, c, 0, 0);
}
