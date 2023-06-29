#![allow(unused)]
// Iterate using a raw pointer in increments of two elements
use std::rc::Rc;
use std::ops::Deref;
fn main() {
    let data = [1u8, 2, 3, 4, 5];
    let ptr = Rc::new(data);
    let step = 2;
    let mut i = 0;
    while i<data.len() {
        println!("{}",ptr.deref()[i]);
        i+=step;
    }

}