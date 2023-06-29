#![allow(unused)]
#![feature(get_mut_unchecked)]

fn main() {
    use std::rc::Rc;

    let x: Rc<&str> = Rc::new("Hello, world!");
    {
        let s = String::from("Oh, no!");
        let mut y: Rc<&str> = x.clone().into();
        unsafe {
            *Rc::get_mut_unchecked(&mut y) = &s;
        }
    }
}