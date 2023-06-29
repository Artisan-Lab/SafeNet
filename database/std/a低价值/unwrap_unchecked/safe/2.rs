#![allow(unused)]
fn main() {
    let x: Option<&str> = None;
    assert_eq!(x.unwrap(), "air"); // Undefined behavior!
}