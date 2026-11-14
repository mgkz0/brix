use crate::memory::PAGE_SIZE;

pub const USERSPACE_START: u64 = 0x0000_0000_0000_0000;
pub const USERSPACE_END: u64 = 0x0000_7FFF_FFFF_FFFF;

pub const KERNELSPACE_START: u64 = 0xFFFF_8000_0000_0000;
pub const KERNELSPACE_END: u64 = 0xFFFF_FFFF_FFFF_FFFF;

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
