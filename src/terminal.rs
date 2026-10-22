use core::fmt;

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
    pub x: usize,
    pub y: usize,
    pub bg: u32,
    pub fg: u32
}

impl<'a> Terminal<'a> {
    pub fn from_kern_fb(fb: &'a KernelFramebuffer) -> Self {
        let width: usize = fb.width / 8;
        let height: usize = fb.height / 8;
        Terminal{
            fb: fb, width: width, height: height, x: 0, y: 0, bg: 0x000000, fg: 0xFFFFFF
        }
    }
    
    pub fn set_color(&mut self, fg: u32, bg: u32) {
        self.fg = fg;
        self.bg = bg;
    }

    pub fn draw_char(
        &self, x: usize, y: usize, ch: char
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
                    self.fg
                } else {
                    self.bg
                };
                self.fb.write(x_fb + coll, y_fb + row, color);
            }
       }
    }

    fn newline(&mut self) {
        self.x = 0;
        self.y += 1;
    }
}


impl fmt::Write for Terminal<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for ch in text.chars() {
            if self.x >= self.width {
                self.newline();
            }
            if self.y >= self.height {
                return Ok(());
            }
            if ch == '\n' {
                self.newline();
                continue;
            }
            self.draw_char(self.x, self.y, ch);
            self.x += 1;
        }
        Ok(())
    }

}
