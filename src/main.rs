#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

pub mod devices;
pub mod memory;
pub mod panic;
pub mod terminal;


use core::panic::PanicInfo;

use limine::BaseRevision;
use limine::request::{FramebufferRequest, MemmapRequest};
use x86_64::instructions::hlt;
use crate::memory::adapters::limine::convert_limine_mmap;
use crate::terminal::framebuffer::KernelFramebuffer;


#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
static MEMORY_MAP_REQUEST: MemmapRequest = MemmapRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    x86_64::instructions::interrupts::enable();
    let fb_req = FRAMEBUFFER_REQUEST.response().unwrap();
    let Some(limine_fb) = fb_req.framebuffers().first() else {
        loop {
            hlt();
        }
    };

    let fb = KernelFramebuffer::new(
        limine_fb.address() as u64, 
        limine_fb.width as usize, 
        limine_fb.height as usize, 
        limine_fb.pitch as usize, 
        limine_fb.memory_model
    );

    let mmap_req = MEMORY_MAP_REQUEST.response().unwrap();
    let mmap = convert_limine_mmap(mmap_req);

    crate::terminal::init_from_kernel_fb(fb, 0x00ff00, 0x000000);
    crate::devices::keyboard::init_keyboard();

    loop {
        hlt();
    }
}
