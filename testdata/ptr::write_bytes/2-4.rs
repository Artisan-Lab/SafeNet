pub fn zero_data(&mut self) {
    unsafe { std::ptr::write_bytes(self.data() as *mut u8, 0, self.nbytes()) }
}
// https://github.com/search?q=ptr%3A%3Awrite_bytes+language%3ARust&type=code&p=3