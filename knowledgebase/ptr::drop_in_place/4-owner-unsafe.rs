#![allow(unused)]
use std::ptr;

fn main() {
    let mut s = String::from("foo");

    unsafe {

        ptr::drop_in_place(&mut s);
    }
}
