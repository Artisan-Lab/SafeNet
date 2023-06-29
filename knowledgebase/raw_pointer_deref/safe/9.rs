#![allow(unused)]

fn main() {
    use std::ptr::NonNull;

    let mut x = 0u32;

    assert_eq!(x, 0);

    x += 2;
    assert_eq!(x, 2);
}