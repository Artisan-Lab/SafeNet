#![allow(unused)]
use std::ptr;

fn bar()-> usize{
    0
}

fn foo(s:&mut Vec<i32>) {
    unsafe {
        let mut s2= ptr::read(&s[bar()]);

        ptr::write(&mut s[bar()] as *mut i32,s2+1);
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