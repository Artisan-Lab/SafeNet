#![allow(unused)]
fn main() {
    let mut x = vec![1, 2, 4];
    if let Some(item) = x.get_mut(1) {
        *item = 13;
        assert_eq!(item, &13);
    }
}
