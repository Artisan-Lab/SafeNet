#![allow(unused)]
use std::ffi::{c_char, CStr};

extern "C" {
    fn my_string() -> *const c_char;
}

fn main() {
    unsafe {
        let slice = CStr::from_ptr(my_string());
    }
}