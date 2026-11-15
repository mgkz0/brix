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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegionType {
    USABLE,
    USED,
    EMPTY,
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
    pub addr_start: u64,
    pub page_count: usize,
    pub ty: RegionType,
}

impl MemReg {
    pub fn new(addr_start: u64, page_count: usize, ty: RegionType) -> Self {
        Self {
            addr_start,
            page_count,
            ty,
        }
    }

    pub const fn empty() -> Self {
        Self {
            addr_start: 0,
            page_count: 0,
            ty: RegionType::EMPTY,
        }
    }
}

pub struct MMap<const N: usize> {
    regions: [MemReg; N],
    count: usize,
}

impl<const N: usize> MMap<N> {
    pub const fn new() -> Self {
        Self {
            regions: [MemReg::empty(); N],
            count: 0,
        }
    }

    pub fn add(&mut self, reg: MemReg) -> Result<(), MemoryError> {
        if self.count >= N {
            return Err(MemoryError::NoEmptyRegs);
        }
        self.regions[self.count] = reg;
        self.count += 1;
        Ok(())
    }

    pub fn get(&self, idx: usize) -> Option<&MemReg> {
        self.entries().get(idx)
    }

    pub fn entries(&self) -> &[MemReg] {
        unsafe { from_raw_parts(self.regions.as_ptr().cast::<MemReg>(), self.count) }
    }
}
