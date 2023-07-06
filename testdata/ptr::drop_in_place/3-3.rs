pub fn clear(&mut self) {
    if self.is_empty() {
        return;
    }

    let mut data = std::ptr::null_mut();
    let mut len = 0;

    std::mem::swap(&mut data, &mut self.data);
    std::mem::swap(&mut len, &mut self.len);

    // SAFETY: At this point, self has been reset to zero so any panics in T's destructor would
    // only leak data not leave the array in bad state.
    unsafe {
        // Call the destructors of all the elements of the old array
        // SAFETY: the slice cannot be used after the call to `drop_in_place`
        std::ptr::drop_in_place(std::slice::from_raw_parts_mut(data, len as usize));
        // Free the data memory where the elements were
        // SAFETY: we have unique access to the data pointer at this point
        // so freeing it is the right thing to do
        crate::imp::CoTaskMemFree(data as _);
    }
}
// https://github.com/microsoft/windows-rs/blob/7b424c2590ac21b47e8ac16b31460fd625e28122/crates/libs/core/src/array.rs#L95