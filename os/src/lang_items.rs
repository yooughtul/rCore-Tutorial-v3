use core::panic::PanicInfo;
use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("[kernel] Panicked at {}:{} {}", location.file(), location.line(), info.message().as_str().unwrap_or("no message"));
    } else {
        println!("[kernel] Panicked: {}", info.message().as_str().unwrap_or("no message"));
    }
    shutdown()
}
