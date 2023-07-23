fn flush(&mut self, valid: bool) -> Result<(Lsn, DiskPtr)> {
    if self.flushed {
        panic!("flushing already-flushed reservation!");
    }

    self.flushed = true;

    if !valid {
        self.buf[4] = MessageKind::Canceled.into();

        // zero the message contents to prevent UB
        #[allow(unsafe_code)]
        unsafe {
            std::ptr::write_bytes(
                self.buf[self.header_len..].as_mut_ptr(),
                0,
                self.buf.len() - self.header_len,
            )
        }
    }
}

// https://github.com/Artisan-Lab/SafeNet/blob/main/knowledgebase/ptr%3A%3Awrite_bytes/1-rawptr-unsafe.rs