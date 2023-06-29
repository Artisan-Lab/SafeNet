#![allow(unused)]
fn main() {
    let smile_utf8 = Box::new([226, 152, 186]);
    let smile = unsafe { std::str::from_boxed_utf8_unchecked(smile_utf8) };
}