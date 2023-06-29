#![allow(unused)]
fn main() {
    let x: Result<u32, &str> = Err("emergency failure");
    unsafe { x.unwrap_unchecked(); } // Undefined behavior!
}