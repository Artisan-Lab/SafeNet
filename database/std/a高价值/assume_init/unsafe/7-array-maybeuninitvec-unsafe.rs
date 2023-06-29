#![allow(unused)]
use std::mem::{self, MaybeUninit};

fn main() {
    let mut data: [MaybeUninit<Vec<u32>>; 1000] = unsafe {
        MaybeUninit::uninit().assume_init()
    };

    for elem in &mut data[..] {
        elem.write(vec![42]);
    }

    let data2 = {unsafe { mem::transmute::<_, [Vec<u32>; 1000]>(data) }
    };
    assert_eq!(&data2[0], &[42]);

}
