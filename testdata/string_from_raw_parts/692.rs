pub unsafe extern "C" fn ffi_reclaim_string(s_ptr: *mut u8, s_len: usize, s_cap: usize) -> isize {
    String::from_raw_parts(s_ptr, s_len, s_cap);
    0
}