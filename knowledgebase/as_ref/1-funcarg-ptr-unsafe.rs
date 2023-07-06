fn process_raw(raw: *mut i32) {
    unsafe {
        if let Some(val_back) = raw.as_ref() {
            println!("We got back the value: {val_back}!");
        }
    }
}