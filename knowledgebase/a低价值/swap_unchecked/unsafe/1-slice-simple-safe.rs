#![allow(unused)]

fn main() {
    let mut v = ["a", "b", "c", "d"];
    v.swap(1, 3);
    assert!(v == ["a", "d", "c", "b"]);
}
