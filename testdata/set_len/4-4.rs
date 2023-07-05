/*
https://github.com/amethyst/specs/blob/89f2826dfafb501b4b55ef53dcb8b721fceb10e5/src/storage/storages.rs#L312
*/
unsafe fn clean<B>(&mut self, has: B)
    where
        B: BitSetLike,
    {
        use std::ptr;
        for (i, v) in self.0.iter_mut().enumerate() {
            if has.contains(i as u32) {
                // drop in place
                ptr::drop_in_place(&mut *v.as_mut_ptr());
            }
        }
        self.0.set_len(0);
    }