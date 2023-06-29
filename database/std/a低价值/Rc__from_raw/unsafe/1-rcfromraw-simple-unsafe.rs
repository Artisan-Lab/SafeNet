#![allow(unused)]
use std::rc::Rc;

fn main() {
    let x = Rc::new("hello".to_owned());
    let x_ptr = Rc::into_raw(x);

    unsafe {
        let x = Rc::from_raw(x_ptr);
    }

}