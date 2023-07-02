#![allow(unused)]
use std::ffi::{c_char, CStr};

extern "C" {
    fn my_string() -> *const c_char;
}

fn main() {
    let raw = unsafe { my_string() };
    let cstr = unsafe { CStr::from_ptr(raw) };
}