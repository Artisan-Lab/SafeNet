fn initialize_value() -> &i32 {
    let mut x = MaybeUninit::<&i32>::uninit();
    unsafe {
        x.as_mut_ptr().write(&0);
        x.assume_init()
    }
}