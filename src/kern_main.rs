use crate::terminal::KernelFramebuffer;
use uefi::mem::memory_map::MemoryMapOwned;
use x86_64::instructions::hlt;

pub fn kernel_main(fb: KernelFramebuffer, _memory_map: MemoryMapOwned) -> ! {
    x86_64::instructions::interrupts::enable();
    crate::terminal::init_from_kernel_fb(fb);
    crate::keyboard::init_keyboard();
    loop {
        hlt()
    }
}
