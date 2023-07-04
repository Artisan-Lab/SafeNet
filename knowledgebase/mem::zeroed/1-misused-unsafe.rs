#![allow(unused)]
#![allow(invalid_value)]
use std::mem;

fn main() {
    let _x: &i32 = unsafe { mem::zeroed() }; // Undefined behavior!
    let _y: fn() = unsafe { mem::zeroed() }; // And again!
}