use crate::boot::limine::{HHDM_ID, LIMINE_REVISION, request::LimineRequest};
use crate::memory::set_dm_offset;
use x86_64::instructions::hlt;


#[used]
#[unsafe(link_section = ".limine_requests")]
pub static HHDM_REQUEST: LimineRequest<HHDMResponse> = LimineRequest::new(HHDM_ID, LIMINE_REVISION);


pub struct HHDMResponse {
    pub offset: u64
}


pub fn init_hhdm_offset() -> u64 {
    let Some(resp) = HHDM_REQUEST.get() else {
        loop {
            hlt();
        }
    };

    resp.offset
 
}
