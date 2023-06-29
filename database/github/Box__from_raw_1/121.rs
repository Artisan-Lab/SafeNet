fn from_box(src: Box<T>) -> Rc<T> {
    unsafe {
        let value_size = size_of_val(&*src);
        let ptr = Self::allocate_for_ptr(&*src);

        // Copy value as bytes
        ptr::copy_nonoverlapping(
            &*src as *const T as *const u8,
            &mut (*ptr).value as *mut _ as *mut u8,
            value_size,
        );

        // Free the allocation without dropping its contents
        let src = Box::from_raw(Box::into_raw(src) as *mut mem::ManuallyDrop<T>);
        drop(src);

        Self::from_ptr(ptr)
    }
}
/*
https://github.com/rust-lang/rust/blob/1d67eba6873b1d551a259a0bbc1e2651b4443e12/library/alloc/src/rc.rs#L1458
*/