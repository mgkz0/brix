pub mod physical;

pub const PAGE_SIZE: u64 = 4096;

#[derive(Debug)]
pub enum MemoryError {
    NoUsableMemory,
}
