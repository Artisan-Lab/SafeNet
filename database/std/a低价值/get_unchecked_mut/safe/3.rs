#![allow(unused)]
fn main() {
    let mut v = "🗻∈🌏";
    assert_eq!("🗻", &v[0..4]);
    assert_eq!("∈", &v[4..7]);
    assert_eq!("🌏", &v[7..11]);
}