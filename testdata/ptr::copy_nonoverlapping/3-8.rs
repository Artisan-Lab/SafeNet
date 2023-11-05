impl CopyOrErr for [u8] {
    fn copy_from_slice_or_err(&mut self, src: &Self) -> Result<(), ErrorCode> {
        if self.len() == src.len() {
            // SAFETY: `self` is valid for `self.len()` elements by definition, and `src` was
            // checked to have the same length. The slices cannot overlap because
            // mutable references are exclusive.
            unsafe {
                ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr(), self.len());
            }
            Ok(())
        } else {
            Err(ErrorCode::SIZE)
        }
    }
}