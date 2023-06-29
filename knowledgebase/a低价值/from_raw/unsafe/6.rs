use std::alloc::{alloc, Layout};
use std::rc::Rc;

fn main() {
    let x = Rc::new(42);
    let x_ptr = Rc::into_raw(x.clone());
    let x = unsafe { Rc::from_raw(x_ptr) };
    assert_eq!(*x, 42);

}