#![allow(unused)]
use std::mem;

fn main() {
    let x: i32 = unsafe { mem::zeroed() };
    assert_eq!(0, x);
}