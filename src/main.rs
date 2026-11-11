#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

pub mod devices;
pub mod kern_main;
pub mod memory;
pub mod panic;
pub mod terminal;


use core::panic::PanicInfo;

use limine::BaseRevision;
use limine::request::{FramebufferRequest, MemmapRequest};
use limine::framebuffer::Framebuffer;
use x86_64::instructions::hlt;


#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
static MEMORY_MAP_REQUEST: MemmapRequest = MemmapRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    loop {
        hlt();
    }
}
