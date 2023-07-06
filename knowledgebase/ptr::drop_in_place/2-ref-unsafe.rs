#![allow(unused)]
use std::ptr;

fn foo(s:&mut String) {
    unsafe {
        ptr::drop_in_place(s);

    }
}
fn main()
{
    
}