fn process_inner_func(b: Box<i32>) {
    let raw = generate(b);
    unsafe {
        let x  = *raw;
    }
}

fn generate(b: Box<i32>) -> *mut i32 {
    Box::into_raw(b)
}