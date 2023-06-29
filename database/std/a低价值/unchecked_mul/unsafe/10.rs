#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroU16;

fn main() -> (){

    let two = NonZeroU16::new(2).unwrap();
    let four = NonZeroU16::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });


}
