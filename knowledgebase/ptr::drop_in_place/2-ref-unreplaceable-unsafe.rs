#![allow(unused)]
use std::ptr;

fn bar()-> usize{
    0
}

fn foo(s:&mut Vec<i32>) {
    unsafe {
        ptr::drop_in_place(s[bar()] as *mut i32);

    }
}
fn main()
{
    let mut a = Vec::new();
    a.push(0);
    foo(&mut a);
    println!("{:?}",a);
}
// cant guarantee index wont be out of range