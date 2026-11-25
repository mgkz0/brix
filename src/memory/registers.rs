use core::arch::asm;

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
