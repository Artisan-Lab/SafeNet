#![allow(unused)]
fn main() {
    use std::cell::UnsafeCell;

    let mut x: UnsafeCell<i32> = 5.into();
// let mut x: UnsafeCell<i32> = UnsafeCell::new(5);

    unsafe {
        // SAFETY: within this scope there are no other references to `x`'s contents,
        // so ours is effectively unique.
        let p1_exclusive: &mut i32 = &mut *x.get(); // -- borrow --+
        *p1_exclusive += 27; //                                     |
    } // <---------- cannot go beyond this point -------------------+


    println!("{:?}", x.get_mut());
}