use std::ptr;
use std::mem;

fn main() {
    let mut v = vec![1, 2, 3];
    let len = v.len();
    for i in 0..len {
        v[i] = i + 4;
    }
    assert_eq!(v, [4, 5, 6]);
}
