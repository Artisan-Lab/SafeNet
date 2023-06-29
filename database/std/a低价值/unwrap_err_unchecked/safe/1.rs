#![allow(unused)]
fn main() {
let x: Result<u32, &str> = Ok(2);
x.unwrap_err() ; // Undefined behavior!
}