#![allow(unused)]
use std::ptr;

fn foo(s:&mut String) {
    unsafe {
        ptr::copy(s.as_mut_ptr(),s.as_mut_ptr(),1);
       
        
    }
}
fn main()
{
    
}