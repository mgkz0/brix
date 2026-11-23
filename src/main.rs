#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

pub mod boot;
pub mod devices;
pub mod memory;
pub mod panic;
pub mod terminal;

use x86_64::instructions::hlt;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    x86_64::instructions::interrupts::enable();
    let fb = boot::limine::framebuffer::init_fb();
    crate::terminal::init_from_kernel_fb(fb, 0x00ff00, 0x000000);
    crate::devices::keyboard::init_keyboard();
    let Some(mm) = boot::limine::mmap::init_mmap() else {
        crate::println!("MemoryMap is None!");
        loop {
            hlt();
        }
    };
    crate::println!("MMAP: {}", mm.entry_count);
    let val = crate::memory::ptables::read_maxphysaddr();
    crate::println!("BITS: {}", val);
    let cr = crate::memory::ptables::read_cr3();
    crate::println!("CR3: {}", cr);
    loop {
        hlt();
    }
}
