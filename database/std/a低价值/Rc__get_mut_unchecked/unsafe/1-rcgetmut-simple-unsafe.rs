#![allow(unused)]
#![feature(get_mut_unchecked)]
use std::rc::Rc;

fn main() {

    let mut x = Rc::new(String::new());
    unsafe {
        Rc::get_mut_unchecked(&mut x).push_str("foo")
    }
}