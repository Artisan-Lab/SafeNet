#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]

use std::mem::MaybeUninit;

fn main(){
    let mut a:[MaybeUninit<u8>; 4] = MaybeUninit::uninit_array();
    a[0].write(0);
    a[1].write(1);
    a[2].write(2);
    a[3].write(3);
    let b = u32::from_le_bytes(unsafe { MaybeUninit::array_assume_init(a) });
}
