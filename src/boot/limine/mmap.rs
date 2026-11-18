use x86_64::instructions::hlt;

use crate::boot::limine::request::LimineResponse;
use crate::boot::limine::{MEMORY_MAP_ID, request::LimineRequest};
use crate::memory::mmap::{MMap, MemReg, RegionType};

pub static MMAP_REQUEST: LimineRequest<MMap> = LimineRequest::new(MEMORY_MAP_ID, 0);

pub fn init_mmap() -> MMap {
    let Some(resp) = MMAP_REQUEST.get() else {
        loop {
            hlt();
        }
    };
    return resp.data;
}
