#![allow(unused)]

fn main() {
    let x = &[1, 2, 4];

    for i in 0..x.len() {
        assert_eq!(x[i], (*x)[i]);
    }
}