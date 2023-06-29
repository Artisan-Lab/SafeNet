#![allow(unused)]
#![feature(layout_for_ptr)]
use std::mem;

fn main() {
    assert_eq!(4, unsafe { mem::align_of_val_raw(&5i32) });
}