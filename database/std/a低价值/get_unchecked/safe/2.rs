#![allow(unused)]
#![feature(slice_ptr_get)]

fn main() {
    let x = &[1, 2, 4];

    assert_eq!(&x[1], &2);
}