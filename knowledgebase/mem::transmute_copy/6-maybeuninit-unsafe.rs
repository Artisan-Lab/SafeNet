#![allow(unused)]
use std::mem::{self, MaybeUninit};

fn main() {
    let mut data: [MaybeUninit<Vec<u32>>; 1000] = unsafe {
        MaybeUninit::uninit().assume_init()
    };

    for elem in &mut data[..] {
        elem.write(vec![42]);
    }

    let mut vec_data: [Vec<u32>; 1000] = unsafe {
        let mut vec_data: [MaybeUninit<Vec<u32>>; 1000] = MaybeUninit::uninit().assume_init();
        for (src, dst) in data.iter().zip(vec_data.iter_mut()) {
            let src_ref = unsafe { src.assume_init_ref() };
            let cloned_vec = src_ref.clone();
            dst.write(cloned_vec);
        }
        mem::transmute_copy(&vec_data)
    };

    assert_eq!(&vec_data[0], &[42]);
}
