#![allow(unused)]
use std::rc::Rc;
use std::ops::Deref;
fn main() {
    let ptr = Rc::new(10u8 as *const u8);
    let val_back = ptr.deref();
    println!("We got back the value: {:?}!", val_back);
}