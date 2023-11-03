#![allow(unused)]
use std::ptr;

fn foo () -> usize{
    0
}

fn main() {
    let mut s = Vec::new();
    s.push(1);
    unsafe {
        let mut s2 = ptr::read(&s[foo()]);

        ptr::write(&mut s[foo()], s2+1);
    }
}

// cant guarantee index wont be out of range