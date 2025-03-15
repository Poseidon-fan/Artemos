use core::panic::PanicInfo;
use log::error;
use sbi::shutdown;
use crate::println;
use crate::syscall::syscall;

// 用于标记 core 中的 panic! 宏要对接的函数
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        error!("[kernel] Panicked: {}", info.message());
    }
    shutdown(true);
}