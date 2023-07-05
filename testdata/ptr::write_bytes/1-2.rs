pub fn clear_bss() {
    extern "C" {
        static mut _data_end: u8;
        static mut _bss_end: u8;
    }
    unsafe {
        let count = &_bss_end as *const u8 as usize - &_data_end as *const u8 as usize;
        ptr::write_bytes(&mut _data_end as *mut u8, 0, count);
    }
}
// https://github.com/o8vm/krabs/blob/4cb5edc8e55a11ef4c54e6b0d22aad225d92b911/src/bios/stage_4th/src/lib.rs#L22