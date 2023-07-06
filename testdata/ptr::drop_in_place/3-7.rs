pub fn remove_columns_at(self, indices: &[usize]) -> OMatrix<T, R, Dyn>
    where
        C: DimSub<Dyn, Output = Dyn>,
        DefaultAllocator: Reallocator<T, R, C, R, Dyn>,
    {
        let mut m = self.into_owned();
        let (nrows, ncols) = m.shape_generic();
        let mut offset: usize = 0;
        let mut target: usize = 0;
        while offset + target < ncols.value() {
            if indices.contains(&(target + offset)) {
                // Safety: the resulting pointer is within range.
                let col_ptr = unsafe { m.data.ptr_mut().add((target + offset) * nrows.value()) };
                // Drop every element in the column we are about to overwrite.
                // We use the a similar technique as in `Vec::truncate`.
                let s = ptr::slice_from_raw_parts_mut(col_ptr, nrows.value());
                // Safety: we drop the column in-place, which is OK because we will overwrite these
                //         entries later in the loop, or discard them with the `reallocate_copy`
                //         afterwards.
                unsafe { ptr::drop_in_place(s) };

                offset += 1;
            } else {
                unsafe {
                    let ptr_source = m.data.ptr().add((target + offset) * nrows.value());
                    let ptr_target = m.data.ptr_mut().add(target * nrows.value());

                    // Copy the data, overwriting what we dropped.
                    ptr::copy(ptr_source, ptr_target, nrows.value());
                    target += 1;
                }
            }
        }
        // https://github.com/dimforge/nalgebra/blob/5baf86b3111858cdecb6518f21ed3b2c579d04f5/src/base/edition.rs#L381