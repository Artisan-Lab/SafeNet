pub unsafe fn update_buffer(&mut self, data: Vec<u8>) {
    let read_ptr = Box::leak(Box::from_raw(self.last_read as *mut u32));
    let write_ptr = Box::leak(Box::from_raw(self.last_write as *mut u32));

    // drop previous buffer
    let prev = Vec::from_raw_parts(self.start as *mut u8, *write_ptr as usize, self.capacity());
    std::mem::drop(prev);

    // write the new buffer information
    let new_ptr = data.as_ptr();
    self.start = new_ptr as i64;
    self.capacity = data.capacity() as _;
    *read_ptr = 0;
    *write_ptr = data.len().saturating_sub(1) as _; // []
    std::mem::forget(data);
}

https://github.com/freenet/locutus/blob/c8586ed72a93b0aa24d549e7b1a4e97bf8d3d025/crates/locutus-stdlib/src/buf.rs#L45