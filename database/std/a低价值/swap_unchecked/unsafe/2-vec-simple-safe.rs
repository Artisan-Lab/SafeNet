#![allow(unused)]

fn main() {
    let mut v = vec!["a", "b", "c", "d"];
    v.swap(1, 3);
    assert!(v == ["a", "d", "c", "b"]);
}
