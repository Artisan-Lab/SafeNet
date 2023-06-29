#![allow(unused)]
use std::ops::Deref;
fn main() {
    use std::rc::Rc;

    let x = Rc::new("hello".to_owned());

    assert_eq!(x.deref(), "hello");
}