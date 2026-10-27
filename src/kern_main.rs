use crate::terminal::{KernelFramebuffer};
use crate::terminal;
use uefi::mem::memory_map::{MemoryMapOwned};


pub fn kernel_main(fb: KernelFramebuffer, memory_map: MemoryMapOwned) -> ! {    
    terminal::init_from_kernel_fb(fb); 
    crate::print!("test {:#x}\nQWERTY\n.!.", 11);
    loop {} 
}
