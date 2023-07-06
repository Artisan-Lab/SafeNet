fn process_inner_func(b1: Box<i32>, b2: Box<i32>) {
    let (raw1, raw2) = generate(b1, b2);
    unsafe {
        let distance = raw1.offset_from(raw2);
    }
}

fn generate(b1: Box<i32>, b2: Box<i32>) -> (*mut i32, *mut i32) {
    (Box::into_raw(b1), Box::into_raw(b2))
}