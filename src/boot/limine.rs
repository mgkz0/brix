use crate::memory::mmap::{MemoryRegion, PAGE_SIZE, RegionType};
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

pub fn convert_limine_region(entry: &memmap::Entry, idx: usize) -> MemoryRegion {
    let reg_type: RegionType = match entry.type_ {
        memmap::MEMMAP_USABLE => RegionType::CONVENTIONAL,
        _ => RegionType::USED,
    };
    MemoryRegion::new(idx, 0, entry.base, entry.length / PAGE_SIZE, reg_type)
}

pub fn init_mmap_from_limine() -> impl Iterator<Item = MemoryRegion> {
    let mmap = MEMORY_MAP_REQUEST.response().unwrap();
    mmap.entries()
        .iter()
        .enumerate()
        .map(|(idx, entry)| convert_limine_region(entry, idx))
}

pub fn init_fb_from_limine() -> KernelFramebuffer {
    let fb_req = FRAMEBUFFER_REQUEST.response().unwrap();
    let Some(limine_fb) = fb_req.framebuffers().first() else {
        loop {
            hlt();
        }
    };

    KernelFramebuffer::new(
        limine_fb.address() as u64,
        limine_fb.width as usize,
        limine_fb.height as usize,
        limine_fb.pitch as usize,
        limine_fb.memory_model,
    )
}
