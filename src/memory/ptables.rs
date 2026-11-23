use core::arch::asm;


#[repr(C, align(4096))]
pub struct PTable {
    data: [u64; 512]
}


pub fn read_cr3() -> u64 {
    let v: u64;
    unsafe {
        asm!(
            "mov {}, cr3", out(reg) v
        )
    }
    v
}


pub fn read_maxphysaddr() -> u8 {
    let v: u32;
    unsafe {
        asm!(
            "mov eax, 0x80000008", "cpuid", out("eax") v
        )
    }
    v as u8
}


pub fn read_cr4() -> u64 {
    let v: u64;
    unsafe {
        asm!(
            "mov {}, cr4", out(reg) v
        )
    }
    v
}


pub fn is_5_lvl_paging() -> bool {
    (read_cr4() & (1 << 12)) != 0
}



pub struct PTM {
}


impl PTM {
    fn get_pml4() {
        let cr3 = read_cr3();
        let max_physaddr = read_maxphysaddr();
        let pml4_addr = cr3 & (((1u64 << max_physaddr) - 1) & !0b111111111111);
    }
}
