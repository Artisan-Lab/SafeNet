unsafe fn from_wide_iter<I: Iterator<Item = u16>>(iter: I, len: u32) -> Result<Self> {
    if len == 0 {
        return Ok(Self::new());
    }

    let ptr = Header::alloc(len)?;

    // Place each utf-16 character into the buffer and
    // increase len as we go along.
    for (index, wide) in iter.enumerate() {
        debug_assert!((index as u32) < len);

        std::ptr::write((*ptr).data.add(index), wide);
        (*ptr).len = index as u32 + 1;
    }

    // Write a 0 byte to the end of the buffer.
    std::ptr::write((*ptr).data.offset((*ptr).len as isize), 0);
    Ok(Self(std::ptr::NonNull::new(ptr)))
}
// https://github.com/microsoft/windows-rs/blob/7b424c2590ac21b47e8ac16b31460fd625e28122/crates/libs/core/src/strings/hstring.rs#L76