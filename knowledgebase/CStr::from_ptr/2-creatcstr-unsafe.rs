#![allow(unused)]
#![feature(const_cstr_methods)]
#![feature(const_cstr_from_ptr)]

use std::ffi::{c_char, CStr};

fn main() {
    const raw: *const c_char = {
        const BYTES: &[u8] = b"Hello, world!\0";
        BYTES.as_ptr().cast()
    };
    const cstr: &CStr = unsafe { CStr::from_ptr(raw) };
}