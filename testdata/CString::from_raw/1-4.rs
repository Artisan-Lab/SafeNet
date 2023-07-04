pub extern "C" fn didkit_free_string(string: *const c_char) {
    if string.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(string as *mut c_char));
    }
}