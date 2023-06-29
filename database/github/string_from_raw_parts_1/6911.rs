pub struct CurlerString {
    raw: *mut u8,
    len: usize,
    capacity: usize,
}

impl Drop for CurlerString {
    fn drop(&mut self) {
        unsafe {
            drop(String::from_raw_parts(self.raw, self.len, self.capacity))
        }
    }
}