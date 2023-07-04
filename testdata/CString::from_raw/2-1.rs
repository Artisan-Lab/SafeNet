fn drop(&mut self) {
    unsafe {
        CString::from_raw(self.kind as *mut i8);
        CString::from_raw(self.display as *mut i8);
        CString::from_raw(self.alt as *mut i8);
        CString::from_raw(self.data as *mut i8);
    }
}