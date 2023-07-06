#![allow(unused)]
use std::ptr;

fn main() {
    let mut s = String::from("foo");

    unsafe {

        ptr::write_bytes(&mut s, 1,1);
    }
}
