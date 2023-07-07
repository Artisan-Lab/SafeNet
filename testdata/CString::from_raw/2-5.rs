pub fn into_c_string(&mut self) -> CString {
    // Transfer ownership of buffer_ptr underlying data.
    let c_string = unsafe { CString::from_raw(self.buffer_ptr) };

    // Set the buffer_ptr to null
    self.buffer_ptr = std::ptr::null_mut();

    c_string
}