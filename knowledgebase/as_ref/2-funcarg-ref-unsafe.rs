fn process_ref(reference: &i32) {
    unsafe {
        let raw = reference as *const i32;
        if let Some(val_back) = raw.as_ref() {
            println!("We got back the value: {val_back}!");
        }
    }
}