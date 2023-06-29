#![allow(unused)]
fn main() {
    let value = -128.9_f64;
    let rounded = unsafe { value.to_int_unchecked::<i8>() };
}