#![allow(unused)]
fn main() {
let x: Result<u32, &str> = Err("emergency failure");
x.unwrap();// Undefined behavior!
}