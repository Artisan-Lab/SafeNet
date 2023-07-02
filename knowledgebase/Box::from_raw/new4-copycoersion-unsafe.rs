/*
    https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw
    Replaceable: No
    Application: drop the content pointed by the pointer.
*/

//#![allow(unused)]

fn foo(x:Box<i32>) -> Box<u32> {
    let y = Box::into_raw(x);
    unsafe{ Box::from_raw(y as *mut u32) }
}

fn main() {
    // Convert a boxed i32 to a boxed u32
    let x1: Box<i32> = Box::new(42);
    let x2 = foo(x1);
    println!("Boxed u32: {}", *x2);
}