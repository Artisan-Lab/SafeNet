#![allow(unused)]
use std::ptr;
use std::rc::Rc;

fn main() {
    let last = Rc::new(1);
    let weak = Rc::downgrade(&last);

    let mut v = vec![Rc::new(0), last];

    unsafe {
        let ptr = &mut v[1] as *mut _;
        v.set_len(1);
        ptr::drop_in_place(ptr);
    }
}