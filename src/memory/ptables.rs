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
        );
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
