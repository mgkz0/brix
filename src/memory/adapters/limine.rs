use limine::{request::MemmapRespData, memmap};
use crate::memory::mmap::{MemoryRegion, PAGE_SIZE, RegionType};


pub fn convert_limine_region(entry: &memmap::Entry, idx: usize) -> MemoryRegion {
    let reg_type: RegionType = match entry.type_ {
        memmap::MEMMAP_USABLE => RegionType::CONVENTIONAL,
        _ => RegionType::USED
    };
    MemoryRegion::new(idx, 0, entry.base, entry.length / PAGE_SIZE, reg_type)
}


pub fn convert_limine_mmap(mmap: &MemmapRespData) -> impl Iterator<Item = MemoryRegion> + '_ {
    mmap.entries().iter().enumerate().map(|(idx, entry)| convert_limine_region(entry, idx))
}


