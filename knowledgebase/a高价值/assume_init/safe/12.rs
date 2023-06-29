#![allow(unused)]
#![feature(allocator_api, new_uninit)]

fn main() {
    let values = Box::<[u32;3]>::new([0,0,0]);

    assert_eq!(*values, [0, 0, 0]);

}