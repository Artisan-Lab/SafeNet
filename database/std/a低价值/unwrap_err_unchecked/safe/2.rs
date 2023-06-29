#![allow(unused)]
fn main() {
let x: Result<u32, &str> = Err("emergency failure");
assert_eq!(x.unwrap_err() , "emergency failure");
}