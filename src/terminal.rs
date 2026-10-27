use core::fmt::{self, Write};

use spin::Mutex;
use uefi::proto::console::gop::{FrameBuffer, ModeInfo, PixelFormat};
use font8x8::{BASIC_FONTS, UnicodeFonts};


pub static TERMINAL: Mutex<Option<Terminal>> = Mutex::new(None);


pub fn init_from_kernel_fb(fb: KernelFramebuffer) {
    let terminal = Terminal::from_kern_fb(fb);
    *TERMINAL.lock() = Some(terminal)
}


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
pub struct Terminal {
    pub fb: KernelFramebuffer,
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub bg: u32,
    pub fg: u32
}

impl Terminal {
    pub fn from_kern_fb(fb: KernelFramebuffer) -> Self {
        for y in 0..fb.height {
            for x in 0..fb.width {
                fb.write(x, y, 0x000000);
            }
        }
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
        &mut self, ch: char
    ) {
        if self.x >= self.width || ch == '\n' {
            self.newline();
        }

        if self.y >= self.height {
            return;
        }

        let Some(glyph) = BASIC_FONTS.get(ch) else {
            return;
        };

        let x_fb = self.x * 8;
        let y_fb = self.y * 8;
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
        self.x += 1;
    }

    fn newline(&mut self) {
        self.x = 0;
        self.y += 1;
    }
}


impl fmt::Write for Terminal {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for ch in text.chars() {
            self.draw_char(ch);
        }
        Ok(())
    }

}


pub fn print_args(args: fmt::Arguments) {
    if let Some(terminal) = TERMINAL.lock().as_mut() {
        terminal.write_fmt(args).unwrap();
    }
}

pub fn print_char(ch: char) {
    if let Some(terminal) = TERMINAL.lock().as_mut() {
        terminal.draw_char(ch);
    }

}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::terminal::print_args(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };}
