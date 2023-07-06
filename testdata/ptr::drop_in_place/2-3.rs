fn drop(&mut self) {
    if self.no_drop {
        return;
    }
    // Dropping a `MaybeUninit` does nothing, thus we need to drop
    // the initialized elements manually, otherwise we may introduce
    // a memory/resource leak if `T: Drop`.
    for elem in &mut self.arr[0..self.init_len] {
        // SAFETY: This is safe, because `self.init_len` represents
        //         exactly the number of initialized elements.
        unsafe {
            ptr::drop_in_place(elem.as_mut_ptr());
        }
    }
}
// https://github.com/graphql-rust/juniper/blob/74710d8a591c99425bb3226f15cbeb66db8036c0/juniper/src/types/containers.rs#L373