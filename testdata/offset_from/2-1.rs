impl<'l, 'r, 'dst, T, G> Drop for BranchlessMergeState<'l, 'r, 'dst, T, G> {
    fn drop(&mut self) {
        unsafe {
            // Extra sanity check.
            let left_len = self
                .left_end
                .offset_from(self.left_begin)
                .try_into()
                .unwrap_abort();
            let right_len = self
                .right_end
                .offset_from(self.right_begin)
                .try_into()
                .unwrap_abort();
            assert_abort(left_len + right_len == self.dst.len());

            // SAFETY: ok by our sanity check.
            let dst_begin = self.dst.begin();
            let mid = dst_begin.add(left_len);
            ptr::copy(self.left_begin, dst_begin, left_len);
            ptr::copy(self.right_begin, mid, right_len);
        }
    }
}
//https://github.com/orlp/glidesort/blob/a9bac7f27179d7d2d97ab6fdebbd42dd6477de3d/src/branchless_merge.rs#L406