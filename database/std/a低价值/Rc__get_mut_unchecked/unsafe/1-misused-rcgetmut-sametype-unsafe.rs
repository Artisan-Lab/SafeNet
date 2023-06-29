#![allow(unused)]
#![feature(get_mut_unchecked)]

fn main() {
    use std::rc::Rc;

    let x: Rc<str> = Rc::from("Hello, world!");
    let mut y: Rc<[u8]> = x.clone().into();
    unsafe {
        Rc::get_mut_unchecked(&mut y).fill(0xff); // 0xff is invalid in UTF-8
    }
}