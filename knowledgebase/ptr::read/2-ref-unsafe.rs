#![allow(unused)]
use std::ptr;

fn foo(s:&mut String) {
    unsafe {
        let mut s2: String = ptr::read(s);
        s2 = String::default();
        ptr::write(s, String::from("bar"));
    }
}
fn main()
{
    
}