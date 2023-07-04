use std::mem;

#[derive(Debug,Default)]
struct MyStruct {
    number: i32,
    flag: bool,
    v: Vec<i32>,
}

fn foo(x: *mut MyStruct) {
    unsafe {(*x).v = vec![1,2,3]; }
}

fn main() {
    // Create an instance of MyStruct initialized with zeroes
    let mut myst: MyStruct = Default::default();
    foo(&mut myst as *mut _);
    println!("{:?}", myst);
}
