#![allow(unused)]

use std::rc::Rc;
fn main() {
    let mut values = Rc::<[u32;3]>::new([0,0,0]);

    assert_eq!(*values, [0, 0, 0]);
}