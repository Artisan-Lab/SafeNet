#![allow(unused)]

fn main() {
let v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = v.split_at(2);
    assert_eq!(left, [1, 2]);
    assert_eq!(right, [3, 4, 5, 6]);
}
