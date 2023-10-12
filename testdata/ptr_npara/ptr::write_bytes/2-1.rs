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

    // zero the crc bytes to prevent UB
    #[allow(unsafe_code)]
    unsafe {
        std::ptr::write_bytes(
            self.buf[..].as_mut_ptr(),
            0,
            std::mem::size_of::<u32>(),
        )
    }

    let crc32 = calculate_message_crc32(
        self.buf[..self.header_len].as_ref(),
        &self.buf[self.header_len..],
    );
    let crc32_arr = u32_to_arr(crc32);

    #[allow(unsafe_code)]
    unsafe {
        std::ptr::copy_nonoverlapping(
            crc32_arr.as_ptr(),
            self.buf.as_mut_ptr(),
            std::mem::size_of::<u32>(),
        );
    }
    self.log.exit_reservation(&self.iobuf)?;

    Ok((self.lsn, self.pointer))
}
}