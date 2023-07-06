#![allow(unused)]

struct R<'a>(&'a i32);

fn main() {
    let x = R(&1);
    let y: R<'static> = unsafe { std::mem::transmute_copy(&x) };
}
