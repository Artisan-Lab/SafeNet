unsafe fn from_raw_parts(buf: *mut u8, length: usize, capacity: usize) -> Self {
    String::from_raw_parts(buf, length, capacity)
}