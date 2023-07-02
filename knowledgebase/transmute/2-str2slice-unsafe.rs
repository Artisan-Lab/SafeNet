#![allow(unused)]

fn main() {
    let s: &str = "hello, world!";
    let slice = unsafe { std::mem::transmute::<&str, &[u8]>(s) };
}
