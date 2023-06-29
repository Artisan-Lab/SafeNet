#![allow(unused)]
use std::rc::Rc;
use std::ops::Deref;
fn main() {
    let s: &str = "123";
    let mut i = 1;
    let ptr2 = Rc::new(s);

    println!("{}",ptr2.deref().chars().nth(i).unwrap());
    i+=1;
    println!("{}",ptr2.deref().chars().nth(i).unwrap());
    i+=1;
}