#![allow(unused)]
use std::ptr;

fn foo(s:&mut String) {
    unsafe {

        ptr::write_bytes(s, 1,1);
    }
}
fn main()
{
    
}