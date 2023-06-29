#![allow(unused)]

fn main() {
    let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
    let slice = "Rust".as_bytes();
    assert_eq!(slice, &[82, 117, 115, 116]);
}
