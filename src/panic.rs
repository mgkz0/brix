use crate::terminal;
use core::fmt::Write;
use core::panic::PanicInfo;
use x86_64::instructions::hlt;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    crate::println!("\nKERNEL_PANIC: {}", info);
    loop {
        hlt();
    }
}
