/*
    This is a modified case from std
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts
    Purpose: change each byte of of the string
    Replaceable? No, because we cannot ensure the modified results are valid utf8s.
*/

#![allow(unused)]
use std::mem;
use std::ptr;

fn main() {

    let s = String::from("hello");
    let mut s = mem::ManuallyDrop::new(s);

    let p = s.as_mut_ptr();
    let len = s.len();
    let capacity = s.capacity();
    for i in 0..len {
        unsafe { ptr::write(p.add(i), ptr::read(p.add(i)) + 1) }; 
    }
    let s = unsafe { String::from_raw_parts(p, len, capacity) }; 

    assert_eq!(String::from("ifmmp"), s);
}
