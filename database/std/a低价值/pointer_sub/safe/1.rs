#![allow(unused)]
use std::rc::Rc;
use std::ops::Deref;
fn main() {
    let s: &str = "123";
    let ptr = Rc::new(s);
    let mut i = 3;
    i-=1;
    println!("{}", ptr.deref().chars().nth(i).unwrap());
    i-=1;
    println!("{}", ptr.deref().chars().nth(i).unwrap());

}