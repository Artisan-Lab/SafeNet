#![allow(unused)]
use std::mem;

#[derive(Default)]
struct MyStruct {
    value: u32,
}

fn main() {
    let mut my_struct: MyStruct = unsafe { mem::zeroed() };
    my_struct.value = 42;
    println!("Value: {}", my_struct.value);
}