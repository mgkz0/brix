use crate::memory::frame_allocator::FrameAllocator;
use crate::terminal::framebuffer::KernelFramebuffer;
use uefi::mem::memory_map::MemoryMapOwned;
use x86_64::instructions::hlt;

pub fn kernel_main(fb: KernelFramebuffer, mm: MemoryMapOwned) -> ! {
    x86_64::instructions::interrupts::enable();
    crate::terminal::init_from_kernel_fb(fb, 0x00ff00, 0x000000);
    crate::devices::keyboard::init_keyboard();
    let _frame_allocator = FrameAllocator::from_mmap(mm);
    loop {
        hlt()
    }
}
