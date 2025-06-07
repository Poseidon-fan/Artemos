use crate::println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[user] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().as_str().ok_or("unknown panic msg").unwrap()
        );
    } else {
        println!(
            "[user] Panicked: {}",
            info.message().as_str().ok_or("unknown panic msg").unwrap()
        );
    }
    loop {}
}
