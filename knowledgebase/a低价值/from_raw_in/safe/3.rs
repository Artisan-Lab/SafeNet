#![allow(unused)]
#![feature(allocator_api)]

fn main() {
    use std::alloc::System;

    let x = Box::new_in(String::from("Hello"), System);

}