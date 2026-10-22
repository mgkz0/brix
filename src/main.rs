#![no_std]
#![no_main]

pub mod terminal;
use uefi::mem::memory_map::{MemoryMapOwned};
use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput};

use crate::terminal::{KernelFramebuffer, Terminal};

fn kernel_main(fb: &KernelFramebuffer, memory_map: MemoryMapOwned) -> ! {
    for y in 0..fb.height {
        for x in 0..fb.width {
            fb.write(x, y, 0x000000);
        }
    }

    let terminal = Terminal::from_framebuffer(fb);
    for (i, ch) in "Pesos".chars().enumerate() {
        terminal.write_char(20 + i, 20, ch, 0x30FF00, 0x000000);
    }    
    loop {
        unsafe {
            core::arch::asm!(
            "xchg bx, bx",
            options(nomem, nostack)
            );
        }
    }
}

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

    kernel_main(&kern_fb, memory_map);

    Status::SUCCESS
}
