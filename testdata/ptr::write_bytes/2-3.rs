unsafe fn write_reserved_indent(&mut self, len: usize) -> Result<(), std::io::Error> {
    unsafe {
        std::ptr::write_bytes(self.buffer_ptr(), b' ', len);
    };
    self.len += len;
    Ok(())
}
// https://github.com/ijl/orjson/blob/314dc9da5b2d2ae515ec25e0fc5b503a4059b472/src/serialize/writer.rs#L208