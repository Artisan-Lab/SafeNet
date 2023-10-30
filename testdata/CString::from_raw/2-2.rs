fn drop(&mut self) {
    unsafe {
        if !self.errMsg.is_null() {
            CString::from_raw(self.errMsg as *mut c_char);
        }
        if self.len > 0 {
            Vec::from_raw_parts(self.data as *mut u8, self.len as usize, self.len as usize);
        }
    }
}