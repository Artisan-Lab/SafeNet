#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroUsize;

fn main() -> (){

    let one = NonZeroUsize::new(1).unwrap();
    let two = NonZeroUsize::new(2).unwrap();

    assert_eq!(two, unsafe { one.unchecked_add(1) });


}
