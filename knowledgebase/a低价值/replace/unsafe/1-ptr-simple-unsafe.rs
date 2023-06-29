#![allow(unused)]
use std::ptr;

fn main() {

    let mut rust = vec!['b', 'u', 's', 't'];

    let b = unsafe {
        ptr::replace(&mut rust[0], 'r')
    };

}