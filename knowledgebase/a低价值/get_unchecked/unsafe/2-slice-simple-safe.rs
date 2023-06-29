#![allow(unused)]
fn main() {
    let x = [1, 2, 4];
    if let Some(item) = x.get(1) {
        assert_eq!(item, &2);
    }
}
