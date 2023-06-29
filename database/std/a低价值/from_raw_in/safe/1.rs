#![allow(unused)]
#![feature(allocator_api)]

fn main() {
    use std::alloc::System;

    let x = Box::new_in(5, System);

}