#![allow(unused)]
use std::ptr;

fn main() {
    let mut s = String::from("foo");

    unsafe {
        let mut s2: String = ptr::read(&s);
        s2 = String::default();
        ptr::write(&mut s, String::from("bar"));
    }
}