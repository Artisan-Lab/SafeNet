#![allow(unused)]
use std::ops::Deref;

fn main() {
    use std::rc::Rc;

    let mut x = Rc::new(String::new());
    
    Rc::get_mut(&mut x).expect("REASON").push_str("foo");

    assert_eq!(*x, "foo");
}