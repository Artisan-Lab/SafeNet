#![allow(unused)]
use std::ptr;

fn main()
{
    unsafe{
        let mut num =42;
        let raw_ptr: *const i32 = &num;
        let a = std::ptr::read(&num);
        raw_ptr.offset_from(&num);
    }
    
}