#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroU8;

fn main() -> (){

    let two = NonZeroU8::new(2).unwrap();
    let four = NonZeroU8::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });


}
