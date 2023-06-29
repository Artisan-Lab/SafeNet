#![allow(unused)]
fn main() {
    let x: Option<&str> = None;
    assert_eq!(unsafe { x.unwrap_unchecked() }, "air"); // Undefined behavior!
}