fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
    let self_bytes = unsafe { self.as_mut_vec() };
    if !self_bytes.is_empty() {
        self_bytes.clear();
    }
    let guard = UTF8Guard { buf: self_bytes };
    let cnt = buf.remaining();
    guard.buf.reserve(cnt);
    guard.buf.put(buf);
    match std::str::from_utf8(guard.buf) {
        Ok(_) => Ok(std::mem::forget(guard)),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
    }
}

// https://github.com/dropbox/pb-jelly/blob/98322e7c61cf395352355860dcaa6a6f91876cd1/pb-jelly/src/base_types.rs#L560