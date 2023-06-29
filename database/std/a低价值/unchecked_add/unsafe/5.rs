#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroU64;

fn main() -> (){

    let one = NonZeroU64::new(1).unwrap();
    let two = NonZeroU64::new(2).unwrap();

    assert_eq!(two, unsafe { one.unchecked_add(1) });


}
