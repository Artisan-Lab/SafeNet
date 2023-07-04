impl ShallowCopy for String {
    unsafe fn shallow_copy(&mut self) -> Self {
        let buf = self.as_bytes_mut().as_mut_ptr();
        let len = self.len();
        let cap = self.capacity();
        String::from_raw_parts(buf, len, cap)
    }
}