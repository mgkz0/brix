use crate::memory::mmap::{INIT_REGIONS_COUNT, MMap, MemReg, PAGE_SIZE, RegionType};
use crate::terminal::framebuffer::KernelFramebuffer;
use limine::{
    BaseRevision, memmap,
    request::{FramebufferRequest, MemmapRequest},
};
use x86_64::instructions::hlt;

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
static MEMORY_MAP_REQUEST: MemmapRequest = MemmapRequest::new();

pub fn convert_limine_region(entry: &memmap::Entry) -> MemReg {
    let reg_type: RegionType = match entry.type_ {
        memmap::MEMMAP_USABLE => RegionType::USABLE,
        _ => RegionType::USED,
    };
    MemReg::new(entry.base, (entry.length / PAGE_SIZE) as usize, reg_type)
}

pub fn init_mmap_from_limine() -> Option<MMap<INIT_REGIONS_COUNT>> {
    let Some(limine_mmap) = MEMORY_MAP_REQUEST.response() else {
        return None;
    };
    let mut kern_mmap = MMap::<INIT_REGIONS_COUNT>::new();

    for (idx, entry) in limine_mmap.entries().iter().enumerate() {
        let reg = convert_limine_region(entry);
        match kern_mmap.add(reg) {
            Ok(()) => continue,
            Err(err) => return None,
        }
    }
    Some(kern_mmap)
}

pub fn init_fb_from_limine() -> Option<KernelFramebuffer> {
    let Some(fb_req) = FRAMEBUFFER_REQUEST.response() else {
        return None;
    };
    let Some(limine_fb) = fb_req.framebuffers().first() else {
        return None;
    };

    Some(KernelFramebuffer::new(
        limine_fb.address() as u64,
        limine_fb.width as usize,
        limine_fb.height as usize,
        limine_fb.pitch as usize,
        limine_fb.memory_model,
    ))
}
