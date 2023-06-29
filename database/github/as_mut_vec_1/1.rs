fn insert_string_at_offset(&mut self, string: String, offset: usize) {
    assert!(offset <= self.data.chars().count());
    let byte_offset = self.byte_offset(offset) as isize;
    let original_size = self.data.len();
    let string_size = string.len();
    self.data.push_str(&string); // first grow organically
    unsafe {
        let bytes_mut = self.data.as_mut_vec().as_mut_ptr();
        // make place by shifting some original data to the side
        ptr::copy(
            self.data.as_bytes().as_ptr().offset(byte_offset),
            bytes_mut.offset(byte_offset + string_size as isize),
            original_size - byte_offset as usize,
        );
        // plunk that chunk in there
        ptr::copy(
            string.as_bytes().as_ptr(),
            bytes_mut.offset(byte_offset),
            string_size,
        );
    }
    self.update_caches();
}
// https://github.com/mathall/rim/blob/5e073ba58d79306fa3a7135554f70370a6ec9fca/src/buffer.rs#L724