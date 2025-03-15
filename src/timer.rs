use riscv::register::time;
use sbi::set_timer;
use crate::config::CLOCK_FREQ;

// 获取时钟计数器的值
pub fn get_time() -> usize {
    time::read()
}

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;

// 计算出 10ms 之内的计数器增量
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

// 以毫秒为单位返回当前计数器的值
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}