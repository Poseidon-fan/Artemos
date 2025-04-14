#![no_std]
pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

pub fn console_getchar() -> usize {
    #[allow(deprecated)]
    sbi_rt::legacy::console_getchar()
}


pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}

// 重新启动，但是不关闭电源
pub fn reboot() -> ! {
    use sbi_rt::{system_reset, WarmReboot, NoReason};
    system_reset(WarmReboot, NoReason);
    unreachable!()
}

// 用来设置 mtimecmp 的值
pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as _);
}