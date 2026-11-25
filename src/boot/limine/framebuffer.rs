use crate::boot::limine::{FRAMEBUFFER_ID, LIMINE_REVISION, request::LimineRequest};
use crate::terminal::framebuffer::Framebuffer;
use x86_64::instructions::hlt;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static FB_REQUEST: LimineRequest<FramebufferResponse> =
    LimineRequest::new(FRAMEBUFFER_ID, LIMINE_REVISION);

#[repr(C)]
pub struct FramebufferResponse {
    pub framebuffer_count: u64,
    pub framebuffers: *const *mut Framebuffer,
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
    unsafe { **fb_ptr }
}
