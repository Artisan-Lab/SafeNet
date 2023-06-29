#![allow(unused)]

// Iterate using a raw pointer in increments of two elements
use std::rc::Rc;
use std::ops::Deref;

fn main() {
    let mut data = [1u8, 2, 3, 4, 5];
    let step = 2;
    let mut i = 0;
    while i < data.len() {
        data[i] = 0;
        // println!("{}", ptr.deref()[i]);
        i += step;
    }
    assert_eq!(&data, &[0, 2, 0, 4, 0]);
}