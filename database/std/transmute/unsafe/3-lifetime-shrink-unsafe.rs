/*
From: https://doc.rust-lang.org/std/mem/fn.transmute.html
*/

#![allow(unused)]

struct R<'a>(&'a i32);
static g:i32 = 1;

fn foo<'a>() {
    let x = R(&g);
    let y = unsafe { std::mem::transmute::<R<'static>, R<'a>>(x) };
}

fn main(){
    foo();
}
