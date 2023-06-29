#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroU8;

fn main() -> (){

    let one = NonZeroU8::new(1).unwrap();
    let two = NonZeroU8::new(2).unwrap();

    assert_eq!(two, unsafe { one.unchecked_add(1) });


}
