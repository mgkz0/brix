pub const PAGE_SIZE: u64 = 4096;

pub struct Frame {
    pub idx: usize,
}

impl Frame {
    pub fn new(addr: u64) -> Self {
        Self {
            idx: (addr / PAGE_SIZE) as usize,
        }
    }

    pub fn addr(&self) -> u64 {
        self.idx as u64 * PAGE_SIZE
    }
}

pub enum RegionType {
    CONVENTIONAL,
    USED,
}

pub struct MemoryRegion {
    pub idx: usize,
    pub current_page: u64,
    pub addr_start: u64,
    pub page_count: u64,
    pub reg_type: RegionType,
}

impl MemoryRegion {
    pub fn new(
        idx: usize,
        current_page: u64,
        addr_start: u64,
        page_count: u64,
        reg_type: RegionType,
    ) -> Self {
        Self {
            idx,
            current_page,
            addr_start,
            page_count,
            reg_type,
        }
    }

    pub fn frame(&self) -> Frame {
        Frame::new(self.addr_start + PAGE_SIZE * self.current_page)
    }
}
