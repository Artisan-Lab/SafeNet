fn my_function() -> &i32 {
    let mut x = MaybeUninit::<&i32>::uninit();
    unsafe {
        x.as_mut_ptr().write(&42);
    }
    let x = unsafe { x.assume_init() };

}