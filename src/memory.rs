use uefi::{
    boot::MemoryType,
    mem::memory_map::{MemoryMap, MemoryMapOwned},
};

pub const PAGE_SIZE: u64 = 4096;

#[derive(Debug)]
pub enum MemoryError {
    NoUsableMemory,
}

pub struct FrameAllocator {
    pub next_frame: u64,
    pub mem_map: MemoryMapOwned,
    index: usize,
    page_count: u64,
    current_page: u64,
}

impl FrameAllocator {
    pub fn from_mmap(mem_map: MemoryMapOwned) -> Result<Self, MemoryError> {
        for (index, m) in mem_map.entries().enumerate() {
            if m.ty != MemoryType::CONVENTIONAL {
                continue;
            }
            let next_frame = m.phys_start;
            let page_count = m.page_count;
            return Ok(Self {
                next_frame,
                mem_map,
                index,
                page_count,
                current_page: 0,
            });
        }
        Err(MemoryError::NoUsableMemory)
    }

    pub fn allocate_frame(&mut self) -> Result<u64, MemoryError> {
        let allocated = self.next_frame;

        if self.current_page + 1 < self.page_count {
            self.next_frame = allocated + PAGE_SIZE;
            self.current_page += 1;
            Ok(allocated)
        } else {
            for (index, m) in self.mem_map.entries().enumerate().skip(self.index + 1) {
                if m.ty != MemoryType::CONVENTIONAL {
                    continue;
                }
                self.next_frame = m.phys_start;
                self.page_count = m.page_count;
                self.current_page = 1;
                self.index = index;
                return Ok(allocated);
            }
            Err(MemoryError::NoUsableMemory)
        }
    }
}
