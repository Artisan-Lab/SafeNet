#![allow(unused)]
use std::{mem::MaybeUninit, ptr};

struct Foo {
    a: u32,
    b: u8,
}

fn main() {
    let foo: Foo = unsafe {
        let mut foo = MaybeUninit::<Foo>::uninit();
        ptr::write(&mut foo.assume_init_mut().a as *mut u32, 1337);

        ptr::write(&mut foo.assume_init_mut().b as *mut u8, 42);

        foo.assume_init()
    };
}