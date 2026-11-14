pub mod physical;
pub mod virt;
pub mod early;
pub mod mmap;
pub mod adapters;


#[derive(Debug)]
pub enum MemoryError {
    NoUsableMemory,
}
