use uefi::proto::console::gop::{FrameBuffer, ModeInfo, PixelFormat};

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
    pub fn from_gop(framebuffer: &mut FrameBuffer, mode_info: &ModeInfo) -> Self {
        let (width, height) = mode_info.resolution();
        KernelFramebuffer {
            address: framebuffer.as_mut_ptr() as u64,
            size: framebuffer.size(),
            width,
            height,
            stride: mode_info.stride(),
            pixel_format: mode_info.pixel_format(),
        }
    }

    pub fn write(&self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.stride + x) * 4;
        let pixel = (self.address as usize + offset) as *mut u32;
        unsafe {
            pixel.write_volatile(color);
        }
    }
}
