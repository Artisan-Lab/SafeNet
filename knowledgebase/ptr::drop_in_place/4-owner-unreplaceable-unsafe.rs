#![allow(unused)]
use std::ptr;

fn foo () -> usize{
    0
}

fn main() {
    let mut s = Vec::new();
    s.push(1);
    unsafe {
        ptr::drop_in_place(s[foo()] as *mut usize);

    }
}

// cant guarantee index wont be out of range