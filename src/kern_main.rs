use crate::terminal::{KernelFramebuffer};
use crate::terminal;
use uefi::mem::memory_map::{MemoryMapOwned};
use x86_64::instructions::hlt;
use x86_64::instructions::port::Port;

pub fn kernel_main(fb: KernelFramebuffer, memory_map: MemoryMapOwned) -> ! {    
    terminal::init_from_kernel_fb(fb);  
    x86_64::instructions::interrupts::enable();
    crate::keyboard::init_keyboard();
    loop {}
}
