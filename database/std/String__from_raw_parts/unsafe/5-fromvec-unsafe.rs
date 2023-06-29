/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts
    Purpose: convert a vec into a string
    Replaceable? Yes
*/

#![allow(unused)]
use std::mem;

fn main() {
    let v = vec![b'h',b'e',b'l',b'l',b'o'];
    let mut v = mem::ManuallyDrop::new(v);
    //let v = vec!['h','e','l','l','o'];
    let p = v.as_mut_ptr();
    let l = v.len();
    let c = v.capacity();
    let mut s = unsafe { String::from_raw_parts(p,l,c) } ;
    assert_eq!(String::from("hello"), s);
    assert_eq!(mem::size_of_val(&s[..]), 5);
}
