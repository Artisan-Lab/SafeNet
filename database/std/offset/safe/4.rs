#![allow(unused)]
use std::rc::Rc;
use std::ops::Deref;
fn main() {
    let mut s = [1, 2, 3];

    let ptr2 = Rc::new(s);

    println!("{}",ptr2.deref()[1]);
    println!("{}",ptr2.deref()[2]);
}