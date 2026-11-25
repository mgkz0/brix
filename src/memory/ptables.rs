use crate::memory::{dm::phys_to_virt, mmap::MMap, physical::PMM, registers};

// PAGE TABLES FLAGS
pub const PRESENT: u64 = 1 << 0;

pub const WRITABLE: u64 = 1 << 1;

pub fn is_present(entry: u64) -> bool {
    entry & PRESENT != 0
}

pub struct PTableEntry {
    pub raw: u64,
}

impl PTableEntry {}

#[repr(C, align(4096))]
#[derive(Clone, Copy, Debug)]
pub struct PTable {
    data: [u64; 512],
}

pub struct TableIndexes {
    pml4: usize,
    pdpt: usize,
    pd: usize,
    pt: usize,
}

impl TableIndexes {
    pub fn new(virt: u64) -> Self {
        Self {
            pml4: ((virt >> 39) & 0b111111111) as usize,
            pdpt: ((virt >> 30) & 0b111111111) as usize,
            pd: ((virt >> 21) & 0b111111111) as usize,
            pt: ((virt >> 12) & 0b111111111) as usize,
        }
    }
}

pub struct PTM {
    pml4_ptr: *mut PTable,
    pmm: PMM,
}

impl PTM {
    pub fn new(pmm: PMM) -> Self {
        let cr3 = registers::read_cr3();
        let max_physaddr = registers::read_maxphysaddr();
        let pml4_phys = cr3 & (((1u64 << max_physaddr) - 1) & !0b111111111111);
        let pml4_virt = phys_to_virt(pml4_phys);
        Self {
            pml4_ptr: pml4_virt as *mut PTable,
            pmm,
        }
    }

    fn new_entry(&mut self, table: &mut PTable, idx: usize) {
        let entry_phys = self.pmm.allocate_frame().unwrap().addr();
        let entry_virt = phys_to_virt(entry_phys);
        unsafe {
            core::ptr::write_bytes(entry_virt as *mut u8, 0, 4096);
        }
        table.data[idx] = entry_phys | PRESENT | WRITABLE;
    }

    pub fn map_page(&mut self, virt: u64) {
        let indexes = TableIndexes::new(virt);
        let frame = self.pmm.allocate_frame().unwrap();
        let mut pml4 = unsafe { *self.pml4_ptr };
        let entry = pml4.data[indexes.pml4];
        if !is_present(entry) {
            self.new_entry(&mut pml4, indexes.pml4);
        }
    }
}
