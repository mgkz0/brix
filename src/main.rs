#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

pub mod kern_main;
pub mod keyboard;
pub mod terminal;

use crate::terminal::KernelFramebuffer;
use kern_main::kernel_main;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;

#[entry]
fn main() -> Status {
    if uefi::helpers::init().is_err() {
        return Status::ABORTED;
    };

    let handle = match boot::get_handle_for_protocol::<GraphicsOutput>() {
        Ok(handle) => handle,
        Err(_) => return Status::NOT_FOUND,
    };

    let mut boot_services = match boot::open_protocol_exclusive::<GraphicsOutput>(handle) {
        Ok(boot_services) => boot_services,
        Err(_) => return Status::NOT_FOUND,
    };

    let mode_info = boot_services.current_mode_info();
    let mut framebuffer = boot_services.frame_buffer();

    let kern_fb = KernelFramebuffer::from_gop(&mut framebuffer, &mode_info);
    let memory_map = unsafe { uefi::boot::exit_boot_services(None) };

    kernel_main(kern_fb, memory_map);
    #[allow(unreachable_code)]
    Status::SUCCESS
}
