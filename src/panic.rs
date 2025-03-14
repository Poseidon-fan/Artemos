use core::panic::PanicInfo;
use sbi::shutdown;
use crate::println;

// 用于标记 core 中的 panic! 宏要对接的函数
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        println!("Panicked: {}", info.message());
    }
    loop{}
}