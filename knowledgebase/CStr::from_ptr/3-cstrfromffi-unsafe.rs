// #![allow(unused)]
// use std::ffi::{c_char, CStr};



fn main() {
    extern "C" {
        fn my_string() -> *const c_char;
    }
    let raw = unsafe { my_string() };
    let cstr = unsafe { CStr::from_ptr(raw) };
}