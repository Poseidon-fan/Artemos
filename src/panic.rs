use core::panic::PanicInfo;

use log::error;

use crate::arch::system;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().as_str().ok_or("unknown panic msg").unwrap()
        );
    } else {
        error!(
            "[kernel] Panicked: {}",
            info.message().as_str().ok_or("unknown panic msg").unwrap()
        );
    }
    system::shutdown();
}
