use uefi::{
    boot::MemoryType,
    mem::memory_map::{MemoryMap, MemoryMapOwned},
};

pub const PAGE_SIZE: u64 = 4096;


pub struct FrameAllocator {
    pub next_frame: Option<u64>,
    pub mem_map: MemoryMapOwned,
    index: usize,
    page_count: u64,
    current_page: u64,
}

impl FrameAllocator {
    pub fn from_mmap(mem_map: MemoryMapOwned) -> Self {
        for (index, m) in mem_map.entries().enumerate() {
            if m.ty != MemoryType::CONVENTIONAL {
                continue;
            }
            let next_frame = Some(m.phys_start);
            let page_count = m.page_count;
            return Self {
                next_frame,
                mem_map,
                index,
                page_count,
                current_page: 0,
            };
        }
        panic!("No usable memory in memory map");
    }

    pub fn allocate_frame(&mut self) -> Option<u64> {
        let allocated = self.next_frame else {
            return None;
        };

        if self.current_page + 1 <= self.page_count {
            self.next_frame = Some(PAGE_SIZE + allocated?);
            self.current_page += 1;
            return allocated;
        } else {
            for (index, m) in self.mem_map.entries().enumerate().skip(self.index + 1) {
                if m.ty != MemoryType::CONVENTIONAL {
                    continue;
                }
                self.next_frame = Some(m.phys_start);
                self.page_count = m.page_count;
                self.current_page = 1;
                self.index = index;
                return allocated;
            }
            self.next_frame = None;
            None
        }
    }
}
