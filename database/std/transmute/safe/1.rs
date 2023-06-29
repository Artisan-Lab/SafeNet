#![allow(unused)]
fn main() {
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];

    let num = u32::from_be_bytes(raw_bytes);
    assert_eq!(num, 0x78563412);
}