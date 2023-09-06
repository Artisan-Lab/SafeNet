fn process_inner_func(b: Box<i32>) {
    let raw = generate(b);
    unsafe {
        if let Some(val_back) = raw.as_ref() {
            println!("We got back the value: {val_back}!");
        }
    }
}

/*
fn generate(b: Box<i32>) -> *mut i32 {
    Box::into_raw(b)
}
*/