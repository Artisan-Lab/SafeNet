/*
    This is a designed case 
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts
    Purpose: construct a string from a mutable str;
    Replaceable? Yes
*/


#![allow(unused)]
use std::mem;

fn foo(s1: &mut str) {
    let p = s1.as_mut_ptr();
    let l = s1.len();
    let s = unsafe { String::from_raw_parts(p, l, l) }; 
    assert_eq!(s, "hello");
}

fn main() {
    let s = String::from("hello");
    let mut s = mem::ManuallyDrop::new(s);
    let s1 = s.as_mut_str();
    foo(s1);
}
