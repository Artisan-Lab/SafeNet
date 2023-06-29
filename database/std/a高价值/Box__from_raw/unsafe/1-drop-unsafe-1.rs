/*
    https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw
    Replaceable: No
    Application: drop the content pointed by the pointer.
*/
#![allow(unused)]

fn drop(p:*mut i32) {
    unsafe { Box::from_raw(p); }
}

fn main() {
    let x = Box::new(5);
    let p = Box::into_raw(x);
    drop(p);
}
