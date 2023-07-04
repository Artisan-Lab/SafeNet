pub fn draw(sel: *mut c_char) {
    unsafe {
        match CString::from_raw(sel).into_string() {
            Ok(sel) => {
                println!("I'll draw on: {}", sel);
                mem::forget(sel);
            },
            Err(_) => ()
        }
    }
}
