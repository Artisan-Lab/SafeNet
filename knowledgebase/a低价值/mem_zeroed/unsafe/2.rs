use std::mem;

struct MyStruct {
    x: i32,
    y: bool,
    z: [u8; 16],
}

fn main() {
    let mut s: MyStruct = unsafe { mem::zeroed() };
}