#![allow(unused)]
#![feature(slice_ptr_get)]

fn main() {
    let x = &mut [1, 2, 4];
    let index = 1;
    unsafe {
        assert_eq!(x.get_unchecked_mut(index) as *mut i32, x.as_mut_ptr().add(index));
    }
}   