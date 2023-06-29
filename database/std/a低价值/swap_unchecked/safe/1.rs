#![allow(unused)]
#![feature(slice_swap_unchecked)]

fn main() {
    let mut v = ["a", "b", "c", "d"];
// SAFETY: we know that 1 and 3 are both indices of the slice
    v.swap(1, 3);
    assert!(v == ["a", "d", "c", "b"]);
}