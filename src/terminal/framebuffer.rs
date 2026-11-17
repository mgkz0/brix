#[derive(Clone, Copy, Debug)]
pub struct Framebuffer {
    pub addr: u64,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub pixel_format: u8,
}

impl Framebuffer {
    pub fn size(&self) -> usize {
        self.pitch * self.height
    }

    pub fn new(addr: u64, width: usize, height: usize, pitch: usize, pixel_format: u8) -> Self {
        return Self {
            addr,
            width,
            height,
            pitch,
            pixel_format,
        };
    }

    pub fn write(&self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = y * self.pitch + x * 4;
        let pixel = (self.addr as usize + offset) as *mut u32;
        unsafe {
            pixel.write_volatile(color);
        }
    }
}
