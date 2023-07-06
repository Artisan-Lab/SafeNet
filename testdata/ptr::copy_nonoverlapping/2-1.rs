impl<T> Drop for CopyOnDrop<'_, T> {
    fn drop(&mut self) {
        // SAFETY:  This is a helper class.
        //          Please refer to its usage for correctness.
        //          Namely, one must be sure that `src` and `dst` does not overlap as required by `ptr::copy_nonoverlapping`.
        unsafe {
            ptr::copy_nonoverlapping(self.src, self.dest, 1);
        }
    }
}
// https://github.com/rayon-rs/rayon/blob/6a9deff7a99e7c7b03bab694bbe8154d6fe2de10/src/slice/quicksort.rs#L41