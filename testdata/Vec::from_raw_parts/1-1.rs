pub unsafe extern "C" fn free_c_args(ptr: *mut *mut c_char, len: c_int) {
    let len = len as usize;
    let v = Vec::from_raw_parts(ptr, len, len);
    for elem in v {
        let s = CString::from_raw(elem);
        std::mem::drop(s);
    }
}

// https://github.com/rustdesk/rustdesk/blob/06af880b126f462591bfcf06c29995c86d3c1852/src/flutter.rs#L136