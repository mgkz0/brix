use crate::memory::{MemoryError, PAGE_SIZE};
use uefi::{
    boot::MemoryType,
    mem::memory_map::{MemoryMap, MemoryMapOwned},
};


pub struct Frame {
    pub idx: usize
}

impl Frame {
    pub fn new(addr: u64) -> Self {Self{idx: (addr / PAGE_SIZE) as usize}}

    pub fn addr(&self) -> u64 {return self.idx as u64 * PAGE_SIZE}
}

pub struct MemoryRegion {
    pub idx: usize,
    pub current_page: u64
}

impl MemoryRegion {
    pub fn new(idx: usize) -> Self {Self{idx, current_page: 0}}

    pub fn from_page(idx: usize, page: u64) -> Self {Self{idx, current_page: page}}
}

pub struct PMM {
    next_frame: Frame,
    mem_map: MemoryMapOwned,
    mem_reg: MemoryRegion,
    bitmap: u64,
}

impl PMM {
    pub fn from_mmap(mem_map: MemoryMapOwned) -> Result<Self, MemoryError> {
        for (reg_idx, mem_reg) in mem_map.entries().enumerate() {
            if mem_reg.ty == MemoryType::CONVENTIONAL {            
                let next_frame = Frame::new(mem_reg.phys_start);
                let bitmap: u64 = 0;
                let mem_reg: MemoryRegion = MemoryRegion::new(reg_idx);
                return Ok(Self {
                    next_frame,
                    mem_map,
                    mem_reg,
                    bitmap,
                });
            }
        }
        Err(MemoryError::NoUsableMemory)
    }

    pub fn allocate_frame(&mut self) -> Result<Frame, MemoryError> {
        let alloc_frame = self.next_frame;
        self.bitmap |= 1u64 << alloc_frame.idx;
        let Some(current_mem_reg) = self.mem_map.get(self.mem_reg.idx) else {
            return Err(MemoryError::NoUsableMemory)
        };

        if self.mem_reg.current_page < current_mem_reg.page_count {
            self.next_frame = Frame::new(alloc_frame.addr() + PAGE_SIZE);
            self.mem_reg.current_page += 1;
            Ok(alloc_frame)
        } else {
            for (reg_idx, mem_reg) in self.mem_map.entries().enumerate().skip(self.mem_reg.idx + 1) {
                if mem_reg.ty == MemoryType::CONVENTIONAL {
                    self.next_frame = Frame::new(mem_reg.phys_start);
                    self.mem_reg = MemoryRegion::new(reg_idx);
                    return Ok(alloc_frame);
                }
            }
            Err(MemoryError::NoUsableMemory)
        }
    }

    pub fn deallocate_frame(&mut self, frame: Frame) {
        self.bitmap &= !(1u64 << frame.idx);
        if frame.idx < self.next_frame.idx {
             for reg_idx in (0..=self.mem_reg.idx).rev() {
                 let Some(mem_reg) = self.mem_map.get(reg_idx) else {continue;};
                 if mem_reg.ty == MemoryType::CONVENTIONAL {
                     let range = mem_reg.phys_start..(mem_reg.phys_start + PAGE_SIZE * mem_reg.page_count);
                     if range.contains(&frame.addr()) {
                        let new_page = (frame.addr() - range.start) / PAGE_SIZE;
                        self.next_frame = frame;
                        self.mem_reg = MemoryRegion::from_page(reg_idx, new_page);
                     }
                            
                 }
             }
        }
    }
}
