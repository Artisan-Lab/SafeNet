/*
    This is a simplified case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_at_mut_unchecked
    Purpose: Divides one slice into two at an index, without doing bounds checking.
    Replaceable? Yes
*/

#![allow(unused)]
#![feature(slice_split_at_unchecked)]

fn main() {
let mut v = [1, 0, 3, 0, 5, 6];
    let (left, right) = unsafe { v.split_at_mut_unchecked(2) };
    assert_eq!(left, [1, 0]);
    assert_eq!(right, [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
    assert_eq!(v, [1, 2, 3, 4, 5, 6]);
}
