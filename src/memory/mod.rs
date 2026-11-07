pub mod frame_allocator;
pub mod virtual_manager;

pub const PAGE_SIZE: u64 = 4096;

#[derive(Debug)]
pub enum MemoryError {
    NoUsableMemory,
}
