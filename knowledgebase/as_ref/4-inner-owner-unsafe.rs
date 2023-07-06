fn process_inner_owner(b: Box<i32>) {
    let raw = Box::into_raw(b);
    unsafe {
        if let Some(val_back) = raw.as_ref() {
            println!("We got back the value: {val_back}!");
        }
    }
}