#![allow(unused)]
#![feature(slice_split_at_unchecked)]

fn main() {
let v = [1, 2, 3, 4, 5, 6];
    let (left, right) = v.split_at(2);
    assert_eq!(left, [1, 2]);
    assert_eq!(right, [3, 4, 5, 6]);
}
