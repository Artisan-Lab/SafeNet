#![allow(unused)]
use std::rc::Rc;


fn main() {
    let s = "123";
    foo(s);
}

pub fn foo(s: &str) -> Rc<Vec<u8>> {
    let v = s.as_bytes().to_vec();
    Rc::from(v)
}
