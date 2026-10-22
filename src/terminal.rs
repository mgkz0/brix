use uefi::proto::console::gop::{FrameBuffer, ModeInfo, PixelFormat};
use font8x8::{BASIC_FONTS, UnicodeFonts};


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
        KernelFramebuffer{
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


#[derive(Clone, Copy, Debug)]
pub struct Terminal<'a> {
    pub fb: &'a KernelFramebuffer,
    pub width: usize,
    pub height: usize,
}

impl<'a> Terminal<'a> {
    pub fn from_framebuffer(fb: &'a KernelFramebuffer) -> Self {
        let width: usize = fb.width / 8;
        let height: usize = fb.height / 8;
        Terminal{
            fb: fb, width: width, height: height
        }
    }

    pub fn write_char(
        &self, x: usize, y: usize, ch: char, fg: u32, bg: u32
    ) {
        if x >= self.width || y >= self.height {
            return;
        }
        let Some(glyph) = BASIC_FONTS.get(ch) else {
            return;
        };

        let x_fb = x * 8;
        let y_fb = y * 8;
        for (row, bit) in glyph.iter().copied().enumerate() {
            for coll in 0..8 {
                let is_set = bit & (1u8 << coll) != 0;
                let color = if is_set {
                    fg
                } else {
                    bg
                };
                self.fb.write(x_fb + coll, y_fb + row, color);
            }
       }
    }
}
