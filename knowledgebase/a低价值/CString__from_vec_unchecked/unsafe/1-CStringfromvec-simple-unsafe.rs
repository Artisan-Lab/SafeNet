#![allow(unused)]
use std::ffi::CString;

fn main() {
    let raw = b"foo".to_vec();
    unsafe {
        let c_string = CString::from_vec_unchecked(raw);
    }
}