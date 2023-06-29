#![allow(unused)]
fn main() {
    use std::cell::UnsafeCell;
    use std::mem::MaybeUninit;

    let m = MaybeUninit::<UnsafeCell<i32>>::uninit();
    unsafe { UnsafeCell::raw_get(m.as_ptr()).write(5); }
    let mut uc = unsafe { m.assume_init() };

    assert_eq!(uc.into_inner(), 5);

}