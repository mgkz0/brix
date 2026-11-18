use crate::memory::{
    MemoryError,
    mmap::{Frame, MMap, MemRegPtr, PAGE_SIZE, RegionType},
};
use core::ptr::addr_of_mut;

// 4 GiB max available bitmap size (For testing)
const MAX_WORDS: usize = 16384;
static mut BITMAP: [u64; MAX_WORDS] = [0; MAX_WORDS];

pub struct PMM {
    mmap: MMap,
    reg_ptr: MemRegPtr,
    bitmap: &'static mut [u64],
}

impl PMM {
    fn is_free(&self, frame_idx: usize) -> bool {
        let bitmap_idx = frame_idx / 64;
        if bitmap_idx >= self.bitmap.len() {
            return false;
        };

        (self.bitmap[bitmap_idx] & (1u64 << (frame_idx % 64))) == 0
    }

    pub fn from_mmap(mmap: MMap) -> Result<Self, MemoryError> {
        for (reg_idx, reg) in mmap.entries().enumerate() {
            if reg.ty() == RegionType::Usable {
                let bitmap = unsafe { &mut *addr_of_mut!(BITMAP) };
                let reg_ptr = MemRegPtr::new(reg_idx, 0, reg.base, reg.length);
                return Ok(Self {
                    mmap,
                    reg_ptr,
                    bitmap,
                });
            }
        }
        Err(MemoryError::NoUsableMemory)
    }

    fn allocate_from_reg(&mut self) -> Option<Frame> {
        while self.reg_ptr.current_page < self.reg_ptr.page_count() {
            let frame = self.reg_ptr.frame();
            self.reg_ptr.current_page += 1;
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
            let range = (self.reg_ptr.idx + 1)..self.mmap.entry_count as usize;
            for reg_idx in range {
                let Some(reg) = self.mmap.get(reg_idx) else {
                    continue;
                };

                if reg.ty() == RegionType::Usable {
                    self.reg_ptr = MemRegPtr::new(reg_idx, 0, reg.base, reg.length);
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
        if frame.idx < self.reg_ptr.frame().idx {
            for reg_idx in (0..=self.reg_ptr.idx).rev() {
                let Some(reg) = self.mmap.get(reg_idx) else {
                    continue;
                };
                if reg.ty() == RegionType::Usable {
                    let range = reg.base..(reg.base + reg.length);
                    if range.contains(&frame.addr()) {
                        let new_page = (frame.addr() - range.start) / PAGE_SIZE;
                        self.reg_ptr =
                            MemRegPtr::new(reg_idx, new_page as usize, reg.base, reg.length);
                        return;
                    }
                }
            }
        }
    }
}
