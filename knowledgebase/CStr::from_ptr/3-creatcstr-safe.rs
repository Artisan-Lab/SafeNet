// #![allow(unused)]
// #![feature(const_cstr_methods)]

// use std::ffi::{c_char, CStr};

fn main() {
    let cstr = CStr::from_bytes_with_nul(b"Hello, world!\0");
    assert!(cstr.is_ok());
}