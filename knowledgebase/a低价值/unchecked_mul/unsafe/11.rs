#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroU32;

fn main() -> (){

    let two = NonZeroU32::new(2).unwrap();
    let four = NonZeroU32::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });


}
