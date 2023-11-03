#![allow(unused)]
use std::ptr;

fn main() {
    let mut s = String::from("foo");

    unsafe {
        ptr::copy(s.as_mut_ptr(),s.as_mut_ptr(),1);

    }
}
