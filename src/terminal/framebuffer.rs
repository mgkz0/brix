#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Framebuffer {
    pub addr: *mut u8,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bits_per_pixel: u64,
}

impl Framebuffer {
    pub fn size(&self) -> usize {
        (self.pitch * self.height) as usize
    }

    pub fn write(&self, x: usize, y: usize, color: u32) {
        if x >= self.width as usize || y >= self.height as usize {
            return;
        }
        let offset = y * self.pitch as usize + x * 4;
        let pixel = (self.addr as usize + offset) as *mut u32;
        unsafe {
            pixel.write_volatile(color);
        }
    }
}

unsafe impl Send for Framebuffer {}
