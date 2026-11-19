use crate::memory::{KERNELSPACE_START, MemoryError, PAGE_SIZE};

pub const EARLY_ALLOC_PAGE_COUNT: usize = 16;

pub struct EarlyVirtAllocator {
    current_page: usize,
    start: u64,
    end: u64,
}

impl EarlyVirtAllocator {
    pub fn new() -> Self {
        return Self {
            current_page: 0, 
            start: KERNELSPACE_START, 
            end: (KERNELSPACE_START + EARLY_ALLOC_PAGE_COUNT as u64 * PAGE_SIZE)
        }
    }

    fn current(&self) -> u64 {
        self.start + self.current_page as u64 * PAGE_SIZE
    }

    pub fn allocate_vm(&mut self, page_count: usize) -> Result<u64, MemoryError> {
        if page_count > EARLY_ALLOC_PAGE_COUNT 
            || page_count == 0 
            || EARLY_ALLOC_PAGE_COUNT < self.current_page + page_count {
            return Err(MemoryError::OutOfMemory)
        }
        let addr = self.current(); 
        self.current_page += page_count;
        Ok(addr)
    }

}
