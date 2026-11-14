#[derive(Clone, Copy, Debug)]
pub struct KernelFramebuffer {
    pub addr: u64,
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub pixel_format: u8,
}

impl KernelFramebuffer {
    pub fn size(&self) -> usize {
        self.stride * self.height
    }

    pub fn new(addr: u64, width: usize, height: usize, stride: usize, pixel_format: u8) -> Self {
        return Self {
            addr,
            width,
            height,
            stride,
            pixel_format,
        };
    }

    pub fn write(&self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.stride + x) * 4;
        let pixel = (self.addr as usize + offset) as *mut u32;
        unsafe {
            pixel.write_volatile(color);
        }
    }
}
