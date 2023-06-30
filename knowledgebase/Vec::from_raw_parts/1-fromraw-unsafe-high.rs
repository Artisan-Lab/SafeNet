/*
    This is a modified case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts 
    Purpose: Creates a new String from a length, capacity, and pointer; a naive case
    Replaceable? No; The original code is nonsense. We don't know the purpose of the code. We mark it as irreplaceable because converting any raw pointer to an owned object is unsafe.
*/

#![allow(unused)]
use std::mem;

fn foo(p:*mut u8, len:usize, cap:usize) {
    let s = unsafe { Vec::from_raw_parts(p, len, cap) }; 
    assert_eq!(Vec::from("hello"), s);
}

fn main() {
    let s = String::from("hello");
    let mut s = mem::ManuallyDrop::new(s);

    let p = s.as_mut_ptr();
    let len = s.len();
    let cap = s.capacity();
   foo(p, len, cap);
}
