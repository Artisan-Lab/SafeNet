pub unsafe extern "C" fn free_demangled_name(buffer: *mut raw::c_char) {
    if buffer.is_null() {
        return;
    }
    ffi::CString::from_raw(buffer);
}