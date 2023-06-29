#![allow(unused)]
fn main() {
    let x = Box::new(5);
    assert_eq!(*x,5)
}