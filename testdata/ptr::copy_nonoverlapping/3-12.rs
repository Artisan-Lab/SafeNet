fn from_box(v: Box<T>) -> Arc<T> {
    unsafe {
        let (box_unique, alloc) = Box::into_unique(v);
        let bptr = box_unique.as_ptr();

        let value_size = size_of_val(&*bptr);
        let ptr = Self::allocate_for_ptr(bptr);

        // Copy value as bytes
        ptr::copy_nonoverlapping(
            bptr as *const T as *const u8,
            &mut (*ptr).data as *mut _ as *mut u8,
            value_size,
        );

        // Free the allocation without dropping its contents
        box_free(box_unique, alloc);

        Self::from_ptr(ptr)
    }
}
// https://github.com/esp-rs/rust/blob/ed3726ba7aea45731260ec8f75f05fc60c2b0f22/library/alloc/src/sync.rs#L1359