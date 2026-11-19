use crate::memory::{
    KERNELSPACE_END, KERNELSPACE_START, PAGE_SIZE, USERSPACE_END, USERSPACE_START,
};

pub struct VMM {
    next_page: u64,
    start_page: u64,
    end_page: u64,
}

impl VMM {
    pub fn new(is_kernel: bool) -> Self {
        if is_kernel {
            return Self {
                next_page: KERNELSPACE_START,
                start_page: KERNELSPACE_START,
                end_page: KERNELSPACE_END,
            };
        }
        return Self {
            next_page: USERSPACE_START,
            start_page: USERSPACE_START,
            end_page: USERSPACE_END,
        };
    }

    pub fn alloc_vm() {}
}
