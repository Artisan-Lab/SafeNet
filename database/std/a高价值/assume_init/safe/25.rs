#![allow(unused)]

fn main() {
    use std::rc::Rc;

    let five = Rc::<u32>::new(5);

    assert_eq!(*five, 5)
}