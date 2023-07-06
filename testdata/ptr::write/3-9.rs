unsafe fn push_unchecked(&mut self, value: T) {
    debug_assert!(self.capacity() > self.len());
    let end = self.as_mut_ptr().add(self.len());
    std::ptr::write(end, value);
    self.set_len(self.len() + 1);
}
// https://github.com/pola-rs/polars/blob/d6898f7dc1dfcb9b130f89a71a43d39f20458023/polars/polars-utils/src/vec.rs#L62