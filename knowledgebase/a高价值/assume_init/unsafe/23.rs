#![allow(unused)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]

fn main() {
    use std::rc::Rc;

    let mut values = Rc::<[u32]>::new_uninit_slice(3);

// Deferred initialization:
    let data = Rc::get_mut(&mut values).unwrap();
    data[0].write(1);
    data[1].write(2);
    data[2].write(3);

    let values = unsafe { values.assume_init() };

    assert_eq!(*values, [1, 2, 3])
}