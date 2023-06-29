#![allow(unused)]

fn main() {
    use std::rc::Rc;

    let zero = Rc::<u32>::new(0);

    assert_eq!(*zero, 0)
}