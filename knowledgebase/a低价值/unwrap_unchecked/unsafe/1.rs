#![allow(unused)]
fn main() {
let x = Some("air");
assert_eq!(unsafe { x.unwrap_unchecked() }, "air");
}