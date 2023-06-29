#![allow(unused)]
use std::ffi::{CStr, CString};

fn main() {
    unsafe {
        let cstring = CString::new("hello").expect("CString::new failed");
        let cstr = CStr::from_bytes_with_nul_unchecked(cstring.to_bytes_with_nul());
    }
}