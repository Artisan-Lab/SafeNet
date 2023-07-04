fn write_the_last_error_into_a_buffer() {
    clear_last_error();

    let err_msg = FFIError::NullPointer.to_string();

    let () = update_last_error(FFIError::NullPointer);

    use std::ffi::CString;
    let buffer = CString::new("").unwrap();

    let mut buffer_raw = CString::into_raw(buffer);
    let output = unsafe {
        let buffer_ptr = &mut buffer_raw as *mut *mut i8 as *mut c_void;
        let _ =
            citeproc_rs_last_error_utf8(crate::buffer::cstring::CSTRING_BUFFER_OPS, buffer_ptr);
        CString::from_raw(buffer_raw)
    };

    let bytes_written = output.as_bytes_with_nul().len();
    assert!(bytes_written > 0);
    assert_eq!(bytes_written, err_msg.len() + 1);

    let msg = str::from_utf8(output.as_bytes()).unwrap();
    assert_eq!(msg, err_msg);
}