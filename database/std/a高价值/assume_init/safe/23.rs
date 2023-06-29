#![allow(unused)]

use std::rc::Rc;
fn main() {
    let mut values = Rc::<[u32;3]>::new([1,2,3]);

    assert_eq!(*values, [1, 2, 3]);
}