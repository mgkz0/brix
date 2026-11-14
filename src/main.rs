#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

pub mod boot;
pub mod devices;
pub mod memory;
pub mod panic;
pub mod terminal;

use crate::boot::limine::{init_fb_from_limine, init_mmap_from_limine};
use crate::terminal::framebuffer::KernelFramebuffer;
use x86_64::instructions::hlt;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    x86_64::instructions::interrupts::enable();
    let fb = init_fb_from_limine();
    let mmap = init_mmap_from_limine();
    crate::terminal::init_from_kernel_fb(fb, 0x00ff00, 0x000000);
    crate::devices::keyboard::init_keyboard();
    crate::println!("TEST LIMINE!!!!");
    loop {
        hlt();
    }
}
