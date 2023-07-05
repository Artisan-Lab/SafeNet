fn drop(&mut self) {
    /* This code gets executed when `same_bucket` panics */

    /* SAFETY: invariant guarantees that `read - write`
     * and `len - read` never overflow and that the copy is always
     * in-bounds. */
    unsafe {
        let ptr = self.vec.as_mut_ptr();
        let len = self.vec.len();

        /* How many items were left when `same_bucket` panicked.
         * Basically vec[read..].len() */
        let items_left = len.wrapping_sub(self.read);

        /* Pointer to first item in vec[write..write+items_left] slice */
        let dropped_ptr = ptr.add(self.write);
        /* Pointer to first item in vec[read..] slice */
        let valid_ptr = ptr.add(self.read);

        /* Copy `vec[read..]` to `vec[write..write+items_left]`.
         * The slices can overlap, so `copy_nonoverlapping` cannot be used */
        ptr::copy(valid_ptr, dropped_ptr, items_left);

        /* How many items have been already dropped
         * Basically vec[read..write].len() */
        let dropped = self.read.wrapping_sub(self.write);

        self.vec.set_len(len - dropped);
    }
}
// https://github.com/cnosdb/cnosdb/blob/f3c68ddc4f32a83b0bee6e244702f389ac0a60af/common/utils/src/dedup.rs#L75