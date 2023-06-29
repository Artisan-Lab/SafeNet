#![allow(unused)]
#![feature(slice_ptr_get)]

fn main() {
    let x = &mut [1, 2, 4];
    let index = 1;

    assert_eq!(x[index], (*x)[index]);
}