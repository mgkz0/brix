#![no_std]
#![no_main]

use uefi::mem::memory_map::{MemoryMapOwned};
use uefi::prelude::*;
use uefi::proto::console::gop::{FrameBuffer, GraphicsOutput, ModeInfo, PixelFormat};


#[derive(Clone, Copy, Debug)]
pub struct KernelFramebuffer {
    pub address: u64,
    pub size: usize,
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub pixel_format: PixelFormat,
}

impl KernelFramebuffer {
    pub fn from_gop(
        framebuffer: &mut FrameBuffer,
        mode_info: &ModeInfo
    ) -> Self {
        let (width, height) = mode_info.resolution();
        Self{
            address: framebuffer.as_mut_ptr() as u64,
            size: framebuffer.size(),
            width: width,
            height: height,
            stride: mode_info.stride(),
            pixel_format: mode_info.pixel_format()
        }
    }

    pub fn write(&self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {return}
        let offset = (y * self.stride + x) * 4;
        let pixel = (self.address as usize + offset) as *mut u32;
        unsafe {
            pixel.write_volatile(color);
        }
    }
}

fn kernel_main(fb: &KernelFramebuffer, memory_map: MemoryMapOwned) -> ! {
    for y in 0..fb.height {
        for x in 0..fb.width {
            fb.write(x, y, 0xFFFF_FFFF);
        }
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
