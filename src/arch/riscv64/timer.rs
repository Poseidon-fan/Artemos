use riscv::register::time;

use crate::arch::{config::CLOCK_FREQ, sbi::set_timer};

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
#[allow(unused)]
const USEC_PER_SEC: usize = 1000000;
const NSEC_PER_SEC: usize = 1000000000;

/// get current time
pub fn get_time() -> usize {
    time::read()
}
/// get current time in microseconds
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

/// set the next timer interrupt
pub fn set_next_trigger() {
    const TIME_SLICE: usize = CLOCK_FREQ / TICKS_PER_SEC;
    set_timer(get_time() + TIME_SLICE);
}
