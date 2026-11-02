use core::fmt::Write;
use uefi::{
    boot::MemoryType,
    mem::memory_map::{MemoryMap, MemoryMapOwned},
};

pub struct MemoryRegion {
    start: u64,
    end: u64,
}

pub fn init_from_mmap(mm: MemoryMapOwned) {
    for m in mm.entries() {
        if m.ty != MemoryType::CONVENTIONAL {
            continue;
        }

        let start = m.phys_start;
        let size = m.page_count * 4096;
        let end = m.phys_start + size;

        crate::println!("START: {start:#X}, END: {end:#X}, SIZE: {size:#X}");
    }
}
