fn process_inner_owner(b: Box<i32>) {
    let raw = Box::into_raw(b);
    unsafe {
        let x  = *raw;
    }
}