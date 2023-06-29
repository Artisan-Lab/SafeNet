#![allow(unused)]
fn main() {
    use std::char;

    let c = char::from_u32(0x2764);

    assert_eq!('❤', c);
}