/*
    https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw
    Replaceable: No
    Application: drop the content pointed by the pointer.
*/

//#![allow(unused)]

struct MyStruct1 { v:Vec<u8> }
struct MyStruct2 { v:Vec<u8> }

fn foo(x:Box<MyStruct1>) -> Box<MyStruct2> {
    let y = Box::into_raw(x);
    unsafe{ Box::from_raw(y.cast()) }
}

fn main() {
    let x1: Box<MyStruct1> = Box::new(MyStruct1{v:vec![1,2,3]});
    let x2 = foo(x1);
}