#![allow(unused)]
use std::mem;

#[repr(packed)]
struct Foo {
    bar: u8,
}

fn main() {
    let foo_array = [10u8];

    unsafe {
        let mut foo_struct: Foo = mem::transmute_copy(&foo_array);
    }

}