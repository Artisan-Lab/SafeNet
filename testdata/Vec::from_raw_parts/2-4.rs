fn drop(&mut self) {
    // Reform the vector and drop it.
    unsafe {
        drop(Vec::from_raw_parts(self.data as *mut u8,
                                 self.data_byte_size as usize,
                                 self.data_byte_size as usize))
    }
}

// https://github.com/pcwalton/rust-media/blob/9fae952f7e2022b4382af2bbe68cfb9937fc2fe9/platform/macos/coreaudio.rs#L59