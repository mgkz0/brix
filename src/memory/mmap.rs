use core::slice::from_raw_parts;

use crate::memory::{MemoryError, mmap::RegionType::AcpiNvs};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    BootloaderReclaimable,
    ExecutableAndModules,
    Framebuffer,
    MappedReserved,
    Unknown,
}

#[derive(Clone, Copy, Debug)]
pub struct MemRegPtr {
    pub idx: usize,
    pub current_page: usize,
    pub base: u64,
    pub length: u64,
}

impl MemRegPtr {
    pub fn new(idx: usize, current_page: usize, base: u64, length: u64) -> Self {
        Self {
            idx,
            current_page,
            base,
            length,
        }
    }

    pub fn page_count(&self) -> usize {
        (self.length / PAGE_SIZE) as usize
    }

    pub fn frame(&self) -> Frame {
        Frame::new(self.base + PAGE_SIZE * self.current_page as u64)
    }
}


#[repr(C)]
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

    pub fn ty(&self) -> RegionType {
        match self.type_ {
            0 => RegionType::Usable,
            1 => RegionType::Reserved,
            2 => RegionType::AcpiReclaimable,
            3 => RegionType::AcpiNvs,
            4 => RegionType::BadMemory,
            5 => RegionType::BootloaderReclaimable,
            6 => RegionType::ExecutableAndModules,
            7 => RegionType::Framebuffer,
            8 => RegionType::MappedReserved,
            _ => RegionType::Unknown,
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MMap {
    pub entry_count: u64,
    entries: *const *mut MemReg,
}

impl MMap {
    pub fn slice(&self) -> &[*mut MemReg] {
        unsafe { core::slice::from_raw_parts(self.entries, self.entry_count as usize) }
    }

    pub fn entries(&self) -> impl Iterator<Item = &MemReg> {
        self.slice().iter().map(|ptr| unsafe { &**ptr })
    }

    pub fn get(&self, idx: usize) -> Option<&MemReg> {
        self.slice().get(idx).map(|ptr| unsafe { &**ptr })
    }
}
