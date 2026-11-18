use x86_64::instructions::hlt;

use crate::boot::limine::request::LimineResponse;
use crate::boot::limine::{MEMORY_MAP_ID, request::LimineRequest};
use crate::memory::mmap::{MMap, MemReg, RegionType};

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static MMAP_REQUEST: LimineRequest<MMap> = LimineRequest::new(MEMORY_MAP_ID, 0);

pub fn init_mmap() -> Option<MMap> {
    let Some(resp) = MMAP_REQUEST.get() else {
        return None
    };
    return Some(resp.data);
}
