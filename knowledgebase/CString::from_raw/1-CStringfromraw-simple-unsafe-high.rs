#![allow(unused)]
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn some_extern_function(s: *mut c_char);
}

fn main() {
    let c_string = CString::new("Hello!").expect("CString::new failed");
    let raw = c_string.into_raw();
    unsafe {
        some_extern_function(raw);
        let c_string = CString::from_raw(raw);
    }
}