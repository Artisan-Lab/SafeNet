#![allow(unused)]
fn main() {
    use std::slice;

// manifest a slice for a single element
    let mut x = 42;
    let mut ptr =&mut x as &mut i32;

    let slice = slice::from_mut(ptr);
    assert_eq!(slice[0], 42);

}