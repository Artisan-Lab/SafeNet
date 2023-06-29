#![allow(unused)]
use std::mem::MaybeUninit;
use std::ptr::addr_of_mut;

#[derive(Debug, PartialEq)]

pub struct Foo {
    name: String,
    list: Vec<u8>,
}

fn main() {
    let foo = {
        let mut uninit: MaybeUninit<Foo> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();
        unsafe { addr_of_mut!((*ptr).name).write("Bob".to_string()); }
        unsafe { addr_of_mut!((*ptr).list).write(vec![0, 1, 2]); }
        unsafe { uninit.assume_init() }
    };

    assert_eq!( foo, Foo {name: "Bob".to_string(), list: vec![0, 1, 2]});
}
