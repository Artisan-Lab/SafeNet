#![allow(unused)]
fn main() {
    let x = &mut [1, 2, 4];
    let index = 1;

    unsafe {
        let elem = x.get_unchecked_mut(index);
        *elem = 13;
    }
    assert_eq!(x, &[1, 13, 4]);
}