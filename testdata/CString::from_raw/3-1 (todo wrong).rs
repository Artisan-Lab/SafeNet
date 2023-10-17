pub unsafe extern "C" fn free_c_args(ptr: *mut *mut c_char, len: c_int) {
    let len = len as usize;

    // Get back our vector.
    // Previously we shrank to fit, so capacity == length.
    let v = Vec::from_raw_parts(ptr, len, len);

    // Now drop one string at a time.
    for elem in v {
        let s = CString::from_raw(elem);
        std::mem::drop(s);
    }

    // Afterwards the vector will be dropped and thus freed.
}

// https://github.com/rustdesk/rustdesk/blob/06af880b126f462591bfcf06c29995c86d3c1852/src/flutter.rs#L140