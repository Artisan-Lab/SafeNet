#![allow(unused)]
fn main() {
    use std::alloc::{alloc, Layout};

    let x = Box::new(5);
    assert_eq!(*x,5)
}