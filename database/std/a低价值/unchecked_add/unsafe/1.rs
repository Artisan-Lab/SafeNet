#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroU16;

fn main() -> (){

    let one = NonZeroU16::new(1).unwrap();
    let two = NonZeroU16::new(2).unwrap();

    assert_eq!(two, unsafe { one.unchecked_add(1) });


}
