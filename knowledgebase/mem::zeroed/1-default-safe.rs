#![allow(unused)]
use std::mem;

#[derive(Default)]
struct MyStruct {
    value: u32,
    flag: bool,
    name: [u8; 10],
}

fn main() {
    let mut my_struct: MyStruct = Default::default();
    my_struct.value = 42;
    println!("Value: {}", my_struct.value);
}