use crate::boot::limine::{LIMINE_REVISION, MEMORY_MAP_ID, request::LimineRequest};
use crate::memory::mmap::MMap;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static MMAP_REQUEST: LimineRequest<MMap> = LimineRequest::new(MEMORY_MAP_ID, LIMINE_REVISION);

pub fn init_mmap() -> Option<MMap> {
    let Some(resp) = MMAP_REQUEST.get() else {
        return None;
    };
    return Some(resp.data);
}
