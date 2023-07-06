#![allow(unused)]

use std::mem;

struct R<'a>(&'a i32);
static G: i32 = 1;

fn foo<'a>() {
    let x = R(&G);
    let y: R<'a> = unsafe { mem::transmute_copy(&x) };
}

fn main() {
    foo();
}
