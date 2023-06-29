#![allow(unused)]
// Undefined behaviour
fn main() {
unsafe { char::from_u32_unchecked(0x110000) };
}