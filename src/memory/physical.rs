use crate::memory::{MemoryError, PAGE_SIZE};
use core::ptr::addr_of_mut;

// 4 GiB max available bitmap size (For testing)
const MAX_WORDS: usize = 16384;
static mut BITMAP: [u64; MAX_WORDS] = [0; MAX_WORDS];

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

pub struct MemoryRegion {
    pub idx: usize,
    pub current_page: u64,
    pub addr_start: u64,
    pub page_count: u64,
}

impl MemoryRegion {
    pub fn new(idx: usize, current_page: u64, addr_start: u64, page_count: u64) -> Self {
        Self {
            idx,
            current_page,
            addr_start,
            page_count,
        }
    }

    pub fn frame(&self) -> Frame {
        Frame::new(self.addr_start + PAGE_SIZE * self.current_page)
    }
}

pub struct PMM {
    mem_map: MemoryMapOwned,
    mem_reg: MemoryRegion,
    bitmap: &'static mut [u64],
}

impl PMM {
    fn is_free(&self, frame_idx: usize) -> bool {
        let bitmap_idx = frame_idx / 64;
        if bitmap_idx > self.bitmap.len() {
            return false;
        };

        (self.bitmap[bitmap_idx] & (1u64 << (frame_idx % 64))) == 0
    }

    pub fn from_mmap(mem_map: MemoryMapOwned) -> Result<Self, MemoryError> {
        for (reg_idx, reg) in mem_map.entries().enumerate() {
            if reg.ty == MemoryType::CONVENTIONAL {
                let bitmap = unsafe { &mut *addr_of_mut!(BITMAP) };
                let mem_reg: MemoryRegion =
                    MemoryRegion::new(reg_idx, 0, reg.phys_start, reg.page_count);
                return Ok(Self {
                    mem_map,
                    mem_reg,
                    bitmap,
                });
            }
        }
        Err(MemoryError::NoUsableMemory)
    }

    fn allocate_from_reg(&mut self) -> Option<Frame> {
        while self.mem_reg.current_page < self.mem_reg.page_count {
            let frame = self.mem_reg.frame();
            self.mem_reg.current_page += 1;
            if self.is_free(frame.idx) {
                self.bitmap[frame.idx / 64] |= 1u64 << (frame.idx % 64);
                return Some(frame);
            }
        }
        None
    }

    pub fn allocate_frame(&mut self) -> Result<Frame, MemoryError> {
        if let Some(frame) = self.allocate_from_reg() {
            return Ok(frame);
        } else {
            let range = (self.mem_reg.idx + 1)..self.mem_map.entries().len();
            for reg_idx in range {
                let Some(reg) = self.mem_map.get(reg_idx) else {
                    continue;
                };

                if reg.ty == MemoryType::CONVENTIONAL {
                    self.mem_reg = MemoryRegion::new(reg_idx, 0, reg.phys_start, reg.page_count);
                    if let Some(frame) = self.allocate_from_reg() {
                        return Ok(frame);
                    }
                }
            }
        }
        Err(MemoryError::NoUsableMemory)
    }

    pub fn deallocate_frame(&mut self, frame: Frame) {
        if self.is_free(frame.idx) {
            return;
        };

        self.bitmap[frame.idx / 64] &= !(1u64 << (frame.idx % 64));
        if frame.idx < self.mem_reg.frame().idx {
            for reg_idx in (0..=self.mem_reg.idx).rev() {
                let Some(reg) = self.mem_map.get(reg_idx) else {
                    continue;
                };
                if reg.ty == MemoryType::CONVENTIONAL {
                    let range = reg.phys_start..(reg.phys_start + PAGE_SIZE * reg.page_count);
                    if range.contains(&frame.addr()) {
                        let new_page = (frame.addr() - range.start) / PAGE_SIZE;
                        self.mem_reg =
                            MemoryRegion::new(reg_idx, new_page, reg.phys_start, reg.page_count);
                        return;
                    }
                }
            }
        }
    }
}
