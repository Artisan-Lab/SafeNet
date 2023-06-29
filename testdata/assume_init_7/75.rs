use std::mem::{self, MaybeUninit};

fn main() {
    let data: [MaybeUninit<Vec<u32>>; 1000] = unsafe {
        MaybeUninit::uninit().assume_init()
    };
}

/*
https://github.com/Warrenren/inside-rust-std-library/blob/432c4e6d526d44382b1dc57cc151dd77445267bb/02-%E5%86%85%E5%AD%98.md?plain=1#L602
*/