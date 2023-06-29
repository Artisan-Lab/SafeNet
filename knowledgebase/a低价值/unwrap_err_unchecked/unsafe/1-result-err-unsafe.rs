#![allow(unused)]
fn main() {
    let x: Result<u32, &str> = Ok(2);
    unsafe { x.unwrap_err_unchecked() }; // Undefined behavior!
}