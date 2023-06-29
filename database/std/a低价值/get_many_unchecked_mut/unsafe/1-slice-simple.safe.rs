#![allow(unused)]
#![feature(get_many_mut)]

fn main() {
    let x = &mut [1, 2, 3, 4];
    if let Ok([a, b, d]) = x.get_many_mut([0, 1, 3]) {
        *a = 10;
        *b = 100;
    }
    assert_eq!(x, &[10, 100, 3, 4]);
}
