fn process_inner_func(b1: Box<i32>, b2: Box<i32>) {
    let raw1 = Box::into_raw(b1);
    let raw2 = Box::into_raw(b2);

    unsafe {
        let distance = raw1.offset_from(raw2);
    }
}