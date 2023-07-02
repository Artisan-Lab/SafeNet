/*
*/

#![allow(unused)]

use std::mem;

fn foo(s:&mut mem::ManuallyDrop<String>) -> *mut u8 {
    s.as_mut_ptr()
}

fn main() {
    let s = String::from("hello");
    let mut s = mem::ManuallyDrop::new(s);
    let p = foo(&mut s);
    let len = s.len();
    let cap = s.capacity();
    let s = unsafe { String::from_raw_parts(p, len, cap) }; 
    assert_eq!(String::from("hello"), s);

}
