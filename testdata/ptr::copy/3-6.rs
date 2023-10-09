pub fn insert_str(&mut self, byte_idx: usize, string: &str) {
    assert!(byte_idx <= self.len());
    assert!(self.as_str().is_char_boundary(byte_idx));

    let len = self.len();
    let amt = string.len();
    self.buffer.reserve(amt);

    // Enlarge buffer and shift bytes over to make room for the
    // insertion string.
    //
    // There are two uses of unsafe here:
    // - `set_len()`, which is used to avoid having to actually
    //   initialize bytes that we know we're immediately going to
    //   write to in the next step.
    // - `ptr::copy()` to efficiently shift bytes over to make room
    //   for the insertion data.  This can be replaced with a safe call
    //   to `copy_within()` on the slice once that API is stabalized in
    //   the standard library.
    unsafe {
        self.buffer.set_len(len + amt);
        ptr::copy(
            self.buffer.as_ptr().add(byte_idx),
            self.buffer.as_mut_ptr().add(byte_idx + amt),
            len - byte_idx,
        );
    }

    // Copy bytes from `string` into the appropriate space in the
    // buffer.
    (&mut self.buffer[byte_idx..(byte_idx + amt)]).copy_from_slice(string.as_bytes());
}
// https://github.com/yatima-inc/ropey/blob/a963a69cd748846ad5810413817178ce28501acd/src/tree/node_text.rs#L312