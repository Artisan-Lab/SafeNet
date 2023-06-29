#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroU32;

fn main() -> (){

    let one = NonZeroU32::new(1).unwrap();
    let two = NonZeroU32::new(2).unwrap();

    assert_eq!(two, unsafe { one.unchecked_add(1) });


}
