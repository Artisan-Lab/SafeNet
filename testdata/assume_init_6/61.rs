fn init_vector() -> Vec<i32> {
    let mut v: MaybeUninit<Vec<i32>> = MaybeUninit::uninit();
    let v_ptr = v.as_mut_ptr();
    unsafe {
        v_ptr.write(vec![1, 2, 3, 4, 5]);
    }

    let initialized_v: Vec<i32> = unsafe {
        v.assume_init()
    };
    initialized_v
}



