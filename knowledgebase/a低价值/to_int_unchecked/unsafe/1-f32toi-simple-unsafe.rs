#![allow(unused)]
fn main() {
    let value = 4.6_f32;
    let rounded = unsafe { value.to_int_unchecked::<u16>() };
}