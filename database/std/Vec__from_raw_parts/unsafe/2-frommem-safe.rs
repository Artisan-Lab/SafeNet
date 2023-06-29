#![allow(unused)]

fn main() {
    let mut vec = Vec::with_capacity(16);
    vec.push(1_000_000);

    assert_eq!(vec, &[1_000_000]);
    assert_eq!(vec.capacity(), 16);
}
