#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroUsize;

fn main() -> (){

    let two = NonZeroUsize::new(2).unwrap();
    let four = NonZeroUsize::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });


}
