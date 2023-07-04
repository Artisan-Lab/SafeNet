pub unsafe fn resize(mut self, sz: usize) -> Vec<MaybeUninit<T>> {
    let len = self.len();

    let new_data = if sz < len {
        self.data.set_len(sz);
        self.data.shrink_to_fit();
        Vec::from_raw_parts(
            self.data.as_mut_ptr() as *mut MaybeUninit<T>,
            self.data.len(),
            self.data.capacity(),
        )
    } else {
        self.data.reserve_exact(sz - len);
        let mut new_data = Vec::from_raw_parts(
            self.data.as_mut_ptr() as *mut MaybeUninit<T>,
            self.data.len(),
            self.data.capacity(),
        );
        new_data.set_len(sz);
        new_data
    };

    std::mem::forget(self);
    new_data
}

// https://github.com/dimforge/nalgebra/blob/5baf86b3111858cdecb6518f21ed3b2c579d04f5/src/base/vec_storage.rs#L127