pub mod early;
pub mod mmap;
pub mod physical;
pub mod virt;

#[derive(Debug)]
pub enum MemoryError {
    NoUsableMemory,
}
