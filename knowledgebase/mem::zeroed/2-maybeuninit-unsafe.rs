use std::mem;

#[derive(Debug)]
struct MyStruct<'a> {
    number: i32,
    flag: bool,
    b: &'a Box<i32>,
}

fn foo<'a>(x: *mut MyStruct<'a>, y: &'a Box<i32>) {
    unsafe {(*x).b = y; }
}

fn main() {
    // Create an instance of MyStruct initialized with zeroes
    let mut myst: MyStruct = unsafe { mem::zeroed() };
    let b = Box::new(1);
    foo(&mut myst as *mut _, &b);
    println!("{:?}", myst);
}
