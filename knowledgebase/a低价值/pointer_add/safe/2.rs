#![allow(unused)]

fn main() {
    let x = &mut [1, 2, 4];
    for i in 0..x.len() {
        x[i] += 2;
    }
    assert_eq!(x, &[3, 4, 6]);
}