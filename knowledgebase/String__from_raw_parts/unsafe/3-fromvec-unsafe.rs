/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts
    Purpose: convert a vec into a string
    Replaceable? Yes
*/

#![allow(unused)]
#![feature(vec_into_raw_parts)]
use std::mem;

fn main() {
    let v = vec![b'h',b'e',b'l',b'l',b'o'];
    let (p,l,c) = v.into_raw_parts();
    let s = unsafe { String::from_raw_parts(p, l, c) }; 
    assert_eq!(String::from("hello"), s);
}
