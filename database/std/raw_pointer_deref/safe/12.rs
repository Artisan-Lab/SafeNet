#![allow(unused)]
use std::rc::Rc;
use std::ops::Deref;
fn main() {
    let ptr = Rc::new(20u8 as *const u8);
    let val_back = ptr.deref();
    println!(val_back);
}