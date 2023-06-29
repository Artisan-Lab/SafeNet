/*
From: https://doc.rust-lang.org/std/mem/fn.transmute.html
*/

#![allow(unused)]

struct R<'a>(&'a i32);

fn main() {
    let x = R(&1);
    let y = unsafe { std::mem::transmute::<_, R<'static>>(x) };
}
