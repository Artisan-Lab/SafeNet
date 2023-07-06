fn input(
    &self,
    mut data: LeasableMutableBuffer<'static, u8>,
) -> Result<(), (ErrorCode, LeasableMutableBuffer<'static, u8>)> {
    let algorithm = if let Some(algorithm) = self.algorithm.extract() {
        algorithm
    } else {
        return Err((ErrorCode::RESERVE, data));
    };

    if TCR(self.descriptor.ctrl.get()).interrupt_enabled() || self.compute_requested.get() {
        // A computation is already in progress
        return Err((ErrorCode::BUSY, data));
    }

    // Need to initialize after checking business, because init will
    // clear out interrupt state.
    self.init();

    // Initialize the descriptor, since it is used to track business
    let len = data.len() as u16;
    let ctrl = TCR::new(true, TrWidth::Byte, len);

    // Make sure we don't try to process more data than the CRC
    // DMA operation supports.
    if data.len() > u16::MAX as usize {
        // Restore the full slice, calculate the current
        // window's start offset.
        let window_ptr = data.as_ptr();
        data.reset();
        let start_ptr = data.as_ptr();
        // Must be strictly positive or zero
        let start_offset = unsafe { window_ptr.offset_from(start_ptr) } as usize;

        // Reslice the buffer such that it start at the same
        // position as the old window, but fits the size
        // constraints
        data.slice(start_offset..=(start_offset + u16::MAX as usize));
    }

    self.enable();

    // Enable DMA interrupt
    self.registers.dmaier.write(DmaInterrupt::DMA::SET);

    // Enable error interrupt
    self.registers.ier.write(Interrupt::ERR::SET);

    // Configure the data transfer descriptor
    //
    // The data length is guaranteed to be <= u16::MAX by the
    // above LeasableMutableBuffer resizing mechanism
    self.descriptor.addr.set(data.as_ptr() as u32);
    self.descriptor.ctrl.set(ctrl.0);
    self.descriptor.crc.set(0); // this is the CRC compare field, not used

    // Prior to starting the DMA operation, drop the
    // LeasableBuffer slice. Otherwise we violate Rust's mutable
    // aliasing rules.
    let full_slice = data.take();
    let full_slice_ptr_len = (full_slice.as_mut_ptr(), full_slice.len());
    self.current_full_buffer.set(full_slice_ptr_len);

    // Ensure the &'static mut slice reference goes out of scope
    //
    // We can't use mem::drop on a reference here, clippy will
    // complain, even though it would be effective at making this
    // 'static mut buffer inaccessible. For now, just make sure to
    // not reference it below.
    //
    // TODO: this needs a proper type and is a broader issue. See
    // tock/tock#2637 for more information.
    //
    // core::mem::drop(full_slice);

    // Set the descriptor memory address accordingly
    self.registers
        .dscr
        .set(&self.descriptor as *const Descriptor as u32);

    // Configure the unit to compute a checksum
    self.registers.mr.write(
        Mode::DIVIDER.val(0)
            + poly_for_alg(algorithm)
            + Mode::COMPARE::CLEAR
            + Mode::ENABLE::Enabled,
    );

    // Enable DMA channel
    self.registers.dmaen.write(DmaEnable::DMAEN::SET);

    Ok(())
}
//https://github.com/tock/tock/blob/6e0aeb0328a114775ce301bac48682173a54a4bd/chips/sam4l/src/crccu.rs#L467