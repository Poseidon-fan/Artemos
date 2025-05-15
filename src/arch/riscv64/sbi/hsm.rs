#![allow(unused)]

use super::sbi_call;
const EID_HSM: usize = 0x48534D;

const FID_HART_START: usize = 0;
const FID_HART_STOP: usize = 1;
const FID_HART_GET_STATUS: usize = 2;
const FID_HART_SUSPEND: usize = 3;

/// start a hart
/// note: mark as `#[inline(never)]` to avoid the compiler to optimize the function call,
/// otherwise, there will be issues when `start_hart` is called in a loop in `boot.rs`.
#[inline(never)]
pub fn start_hart(hartid: usize, start_addr: usize, opaque: usize) -> usize {
    sbi_call(EID_HSM, FID_HART_START, hartid, start_addr, opaque)
}
