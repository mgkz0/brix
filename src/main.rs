#![no_std]
#![no_main]

pub mod terminal;
pub mod kern_main;

use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput};
use kern_main::kernel_main;
use crate::terminal::KernelFramebuffer;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    let mut boot_services = boot::open_protocol_exclusive::<GraphicsOutput>(
        boot::get_handle_for_protocol::<GraphicsOutput>().expect("no GOP display found"),
    ).expect("failed to open GOP");

    let mode_info = boot_services.current_mode_info();
    let mut framebuffer = boot_services.frame_buffer();

    let kern_fb = KernelFramebuffer::from_gop(&mut framebuffer, &mode_info);
    let memory_map = unsafe {
        uefi::boot::exit_boot_services(None)
    };

    kernel_main(kern_fb, memory_map);

    Status::SUCCESS
}
