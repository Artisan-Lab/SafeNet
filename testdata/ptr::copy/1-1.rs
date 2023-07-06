pub extern "C" fn xGetPathname(buf: *mut c_char, orig: *const c_char, orig_len: c_int) {
    unsafe { std::ptr::copy(orig, buf, orig_len as usize) }
    unsafe {
        std::ptr::copy(
            "-wal".as_ptr(),
            (buf as *mut u8).offset(orig_len as isize),
            4,
        )
    }
}
// https://github.com/libsql/libsql/blob/a125b1933b1c0d8cedcc0ee2a78cd83871ce212e/src/rust/libsql-sys/src/wal_hook.rs#L425