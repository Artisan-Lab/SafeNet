fn drop(&mut self) {
    if self.deleted_cnt > 0 {
        // SAFETY: Trailing unchecked items must be valid since we never touch them.
        unsafe {
            ptr::copy(
                self.v.as_ptr().add(self.processed_len),
                self.v.as_mut_ptr().add(self.processed_len - self.deleted_cnt),
                self.original_len - self.processed_len,
            );
        }
    }
    // SAFETY: After filling holes, all items are in contiguous memory.
    unsafe {
        self.v.set_len(self.original_len - self.deleted_cnt);
    }
}
// https://github.com/rust-lang/rust/blob/dfe0683138de0959b6ab6a039b54d9347f6a6355/library/alloc/src/vec/mod.rs#L1610