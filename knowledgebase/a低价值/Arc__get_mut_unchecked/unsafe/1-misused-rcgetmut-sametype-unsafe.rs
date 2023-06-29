#![allow(unused)]
#![feature(get_mut_unchecked)]

fn main() {
    use std::sync::Arc;

    let x: Arc<str> = Arc::from("Hello, world!");
    let mut y: Arc<[u8]> = x.clone().into();
    unsafe {
        Arc::get_mut_unchecked(&mut y).fill(0xff); // 0xff is invalid in UTF-8
    }
}