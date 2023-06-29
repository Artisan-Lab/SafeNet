#![allow(unused)]
#![feature(layout_for_ptr)]
use std::mem;

fn main() {
    let x: [u8; 13] = [0; 13];
    let y: &[u8] = &x;
    assert_eq!(13, unsafe { mem::size_of_val_raw(y) });
}