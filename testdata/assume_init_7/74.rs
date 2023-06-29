use std::mem::{self, MaybeUninit};
fn main() {
    let mut buffer_data: [MaybeUninit<Vec<u8, LinearAllocator>>; Self::BUFFER_COUNT] =
    unsafe { MaybeUninit::uninit().assume_init() };
}
/*
https://github.com/patataofcourse/Barista/blob/63f92500c2cf474b95973b98ae2df168e94efc5b/src/format/bcstm.rs#L208
*/