// #![allow(unused)]
// use std::ffi::{c_char, CStr};

// extern "C" { fn my_string() -> *const c_char; }

fn foo<'a>(raw: *const c_char) -> &'a CStr {
    unsafe { CStr::from_ptr(raw) }
}  

/*
fn main() {
    let raw = unsafe { my_string() };
    let cstr = foo(raw);
}
*/