use core::slice::from_raw_parts;

use crate::memory::MemoryError;

pub const PAGE_SIZE: u64 = 4096;
pub const INIT_REGIONS_COUNT: usize = 128;

#[derive(Clone, Copy, Debug)]
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

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    Usable = 0,
    Reserved = 1,
    AcpiReclaimable = 2,
    AcpiNvs = 3,
    BadMemory = 4,
    BootloaderReclaimable = 5,
    ExecutableAndModules = 6,
    Framebuffer = 7,
    MappedReserved = 8,
}

#[derive(Clone, Copy, Debug)]
pub struct MemRegPtr {
    pub idx: usize,
    pub current_page: usize,
    pub addr_start: u64,
    pub page_count: usize,
}

impl MemRegPtr {
    pub fn new(idx: usize, current_page: usize, addr_start: u64, page_count: usize) -> Self {
        Self {
            idx,
            current_page,
            addr_start,
            page_count,
        }
    }

    pub fn frame(&self) -> Frame {
        Frame::new(self.addr_start + PAGE_SIZE * self.current_page as u64)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MemReg {
    pub base: u64,
    pub length: u64,
    pub type_: u64,
}

impl MemReg {
    pub fn page_count(&self) -> usize {
        (self.length / PAGE_SIZE) as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MMap {
    entry_count: u64,
    entries: *const *mut MemReg,
}


impl MMap {
    pub fn entries(&self) -> &[*mut MemReg]{
        unsafe {core::slice::from_raw_parts(self.entries, self.entry_count as usize)}
    }

    pub fn get(&self, idx: usize) -> Option<&*mut MemReg> {
        self.entries().get(idx)
    }

}
