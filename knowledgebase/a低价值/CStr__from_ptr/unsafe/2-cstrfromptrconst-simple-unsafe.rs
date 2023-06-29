#![allow(unused)]
#![feature(const_cstr_methods)]
use std::ffi::{c_char, CStr};

fn main() {
    const HELLO_PTR: *const c_char = {
        const BYTES: &[u8] = b"Hello, world!\0";
        BYTES.as_ptr().cast()
    };
    const HELLO: &CStr = unsafe { CStr::from_ptr(HELLO_PTR) };
}