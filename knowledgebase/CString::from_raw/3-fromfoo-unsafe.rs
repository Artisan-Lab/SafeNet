#![allow(unused)]
use std::ffi::CString;
// use std::os::raw::c_char;

// fn foo() -> *mut c_char {
//     let mut cstring = CString::new("hello").expect("Failed to create CString");
//     cstring.into_raw() as *mut c_char
// }  

fn main() {
    let raw = foo();
    unsafe { CString::from_raw(raw); }
}
