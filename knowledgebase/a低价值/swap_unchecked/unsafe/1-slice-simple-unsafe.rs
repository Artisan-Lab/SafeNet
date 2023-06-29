/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap_unchecked
    Purpose: swap two items of an Vec
    Replaceable? Yes
*/

#![allow(unused)]
#![feature(slice_swap_unchecked)]

fn main() {
    let mut v = ["a", "b", "c", "d"];
    unsafe { v.swap_unchecked(1, 3) };
    assert!(v == ["a", "d", "c", "b"]);
}
