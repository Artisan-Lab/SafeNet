#![allow(unused)]
use std::rc::Rc;

fn main() {
    let five = Rc::new(5);

    unsafe {
        let ptr = Rc::into_raw(five);
        Rc::increment_strong_count(ptr);
    }
}