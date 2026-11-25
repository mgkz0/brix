use crate::memory::{
    dm::{self, phys_to_virt},
    mmap::{Frame, MMap},
    physical::PMM,
    registers,
};

// PAGE TABLES FLAGS
pub const PRESENT: u64 = 1 << 0;
pub const WRITABLE: u64 = 1 << 1;
pub const USER: u64 = 1 << 2;

#[repr(C, align(4096))]
#[derive(Clone, Copy, Debug)]
pub struct PTable {
    data: [u64; 512],
}

impl PTable {
    pub fn is_present(&self, idx: usize) -> bool {
        self.data[idx] & PRESENT != 0
    }
    pub fn next_virt(&self, idx: usize, max_physaddr: u8) -> u64 {
        phys_to_virt(self.data[idx] & ((1u64 << max_physaddr) - 1) & !0xfff)
    }

    pub fn set_present(&mut self, idx: usize, frame: Frame) {
        self.data[idx] = frame.addr() | PRESENT | WRITABLE;
    }
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
    pub max_physaddr: u8,
    pub pmm: PMM,
    pml4_ptr: *mut PTable,
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
            max_physaddr,
        }
    }

    fn get_or_create_table_virt(&mut self, parent_pt: &mut PTable, idx: usize) -> u64 {
        if !parent_pt.is_present(idx) {
            let parent_frame = self.pmm.allocate_frame().unwrap();
            dm::create_direct(parent_frame.addr());
            parent_pt.set_present(idx, parent_frame);
        }

        parent_pt.next_virt(idx, self.max_physaddr)
    }

    pub fn map_page(&mut self, map_virt: u64) {
        let idxs = TableIndexes::new(map_virt);
        let map_frame = self.pmm.allocate_frame().unwrap();

        let pml4 = unsafe { &mut *self.pml4_ptr };

        let pdpt_virt = self.get_or_create_table_virt(pml4, idxs.pml4);
        let pdpt = unsafe { &mut *(pdpt_virt as *mut PTable) };

        let pd_virt = self.get_or_create_table_virt(pdpt, idxs.pdpt);
        let pd = unsafe { &mut *(pd_virt as *mut PTable) };

        let pt_virt = self.get_or_create_table_virt(pd, idxs.pd);
        let pt = unsafe { &mut *(pt_virt as *mut PTable) };

        pt.set_present(idxs.pt, map_frame);
    }
}
