#![allow(unused)]
use std::rc::Rc;
use std::ops::Deref;
fn main() {
    let s: &str = "123";

    let ptr2 = Rc::new(s);

    println!("{}",ptr2.deref().chars().nth(1).unwrap());
    println!("{}",ptr2.deref().chars().nth(2).unwrap());
}