#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroIsize;

fn main() -> (){

    let two = NonZeroIsize::new(2).unwrap();
    let four = NonZeroIsize::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });


}
