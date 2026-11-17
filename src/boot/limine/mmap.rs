use crate::boot::limine::{MEMORY_MAP_ID, request::LimineRequest};

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimineRegType {
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

pub struct LimineMemReg {
    pub base: u64,
    pub length: u64,
    pub type_: u64,
}

pub struct LimineMemoryMap {
    entry_count: u64,
    entries: *const *mut LimineMemReg,
}

pub static MMAP_REQUEST: LimineRequest<LimineMemoryMap> = LimineRequest::new(MEMORY_MAP_ID, 0);
