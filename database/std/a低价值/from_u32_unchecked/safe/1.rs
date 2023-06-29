#![allow(unused)]
// Undefined behaviour
fn main() {
    char::from_u32(0x110000);
}