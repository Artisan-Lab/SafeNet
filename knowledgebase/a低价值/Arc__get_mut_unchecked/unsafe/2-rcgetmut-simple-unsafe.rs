#![allow(unused)]
#![feature(get_mut_unchecked)]
use std::sync::Arc;

fn main() {
    let mut x = Arc::new(String::new());
    unsafe {
        Arc::get_mut_unchecked(&mut x).push_str("foo")
    }
}