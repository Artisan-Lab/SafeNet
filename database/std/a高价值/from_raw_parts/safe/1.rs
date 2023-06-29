#![allow(unused)]
fn main() {
    use std::slice;

    let x = 42;
    let slice = slice::from_ref(&x);
    assert_eq!(slice[0], 42);
}