#![allow(unused)]
fn main() {
    use std::rc::Rc;

    let x = Rc::new("hello".to_owned());
    let x_ptr = Rc::into_raw(x);
    assert_eq!(unsafe { &*x_ptr }, "hello");
}