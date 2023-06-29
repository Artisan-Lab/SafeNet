#![allow(unused)]
fn main() {
    use std::char;
    let c = unsafe { char::from_u32_unchecked(0x2764) };

    assert_eq!('❤', c);
}