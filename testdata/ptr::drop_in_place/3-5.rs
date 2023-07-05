pub(super) fn forget_allocation_drop_remaining(&mut self) {
    let remaining = self.as_raw_mut_slice();

    // overwrite the individual fields instead of creating a new
    // struct and then overwriting &mut self.
    // this creates less assembly
    self.cap = 0;
    self.buf = unsafe { NonNull::new_unchecked(RawVec::NEW.ptr()) };
    self.ptr = self.buf.as_ptr();
    self.end = self.buf.as_ptr();

    // Dropping the remaining elements can panic, so this needs to be
    // done only after updating the other fields.
    unsafe {
        ptr::drop_in_place(remaining);
    }
}
// https://github.com/Rust-for-Linux/linux/blob/c8d1ae2cbe240789ad402c71fce78a7ea1ebdea5/rust/alloc/vec/into_iter.rs#L134