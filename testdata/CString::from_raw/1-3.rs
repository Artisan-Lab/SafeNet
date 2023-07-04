pub unsafe extern "C" fn wr_api_free_error_msg(msg: *mut c_char) {
    if !msg.is_null() {
        drop(CString::from_raw(msg));
    }
}