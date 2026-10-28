use crate::terminal::{KernelFramebuffer};
use crate::terminal;
use uefi::mem::memory_map::{MemoryMapOwned};
use x86_64::instructions::hlt;
use x86_64::instructions::port::Port;

pub fn kernel_main(fb: KernelFramebuffer, memory_map: MemoryMapOwned) -> ! {    
    terminal::init_from_kernel_fb(fb); 
    crate::keyboard::init_idt();

    unsafe {
        crate::keyboard::PICS.lock().initialize();
    }

    x86_64::instructions::interrupts::enable();

    unsafe {
        let mut master_mask_port = Port::<u8>::new(0x21);
        let mut mask = master_mask_port.read();
        mask &= !(1 << 1); // ALLOW IRQ Mask Bit for keyboard interrupts
        master_mask_port.write(mask); 
    }
    loop {
    } 
}
