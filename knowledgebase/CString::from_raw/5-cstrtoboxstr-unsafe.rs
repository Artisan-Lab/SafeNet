use std::ffi::{CStr, CString};

fn main() {
    let x = CString::new("Hello, World!").expect("CString::new failed").into_raw();
    let y = unsafe { CString::from_raw(x) };
    let z: Box<str> = y.into_string().expect("Invalid UTF-8").into_boxed_str();
}
