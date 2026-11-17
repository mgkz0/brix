use x86_64::instructions::hlt;
use crate::terminal::framebuffer::Framebuffer;
use crate::boot::limine::{FRAMEBUFFER_ID, request::LimineRequest};

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static FB_REQUEST: LimineRequest<FramebufferResponse> =
    LimineRequest::new(FRAMEBUFFER_ID, 0);


#[repr(C)]
pub struct FramebufferResponse {
    pub framebuffer_count: u64,
    pub framebuffers: *const *mut LimineFramebuffer 
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct LimineFramebuffer {
    pub addr: *mut u8,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bits_per_pixel: u16,
    pub memory_model: u8
}


impl LimineFramebuffer {
    pub fn to_kernel(&self) -> Framebuffer {
        Framebuffer::new(self.addr as u64, self.width as usize, self.height as usize, self.pitch as usize, self.memory_model)
    }
}


pub fn init_fb() -> Framebuffer {
    let Some(resp) = FB_REQUEST.get() else {
        loop {
            hlt();
        }
    };
    let fb_ptr = resp.framebuffers;
    if resp.framebuffer_count == 0 || resp.framebuffers.is_null() {
        loop {
            hlt();
        }
    } 
    unsafe { &**fb_ptr }.to_kernel()
}
