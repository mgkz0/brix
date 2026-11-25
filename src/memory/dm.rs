pub static mut DM_OFFSET: u64 = 0;

pub fn set_dm_offset(offset: u64) {
    unsafe { DM_OFFSET = offset };
}

pub fn phys_to_virt(addr: u64) -> u64 {
    unsafe { DM_OFFSET + addr }
}
