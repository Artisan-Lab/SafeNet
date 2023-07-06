#![allow(unused)]
use std::ptr;

fn foo () -> usize{
    0
}

fn main() {
    let mut s = Vec::new();
    s.push(1);
    unsafe {
        ptr::copy_nonoverlapping(s.as_mut_ptr().add(foo ()),s.as_mut_ptr().add(foo ()),1);

        
    }
}

// cant guarantee index wont be out of range