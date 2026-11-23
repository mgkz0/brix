pub mod early;
pub mod heap;
pub mod mmap;
pub mod physical;
pub mod virt;
pub mod ptables;


#[derive(Debug)]
pub enum MemoryError {
    NoUsableMemory,
    OutOfMemory,
}

pub const PAGE_SIZE: u64 = 4096;

pub const USERSPACE_START: u64 = 0x0000_0000_0000_0000;
pub const USERSPACE_END: u64 = 0x0000_7FFF_FFFF_FFFF;

pub const KERNELSPACE_START: u64 = 0xFFFF_8000_0000_0000;
pub const KERNELSPACE_END: u64 = 0xFFFF_FFFF_FFFF_FFFF;


pub static mut DM_OFFSET: u64 = 0;


pub fn set_dm_offset(offset: u64) {
    unsafe { DM_OFFSET = offset };
}
