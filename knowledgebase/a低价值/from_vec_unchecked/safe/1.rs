#![allow(unused)]

fn main() {
    let s = "Löwe 老虎 Léopard";

    assert_eq!("Löwe 老虎 Léopard", &s[0..21]);

    let s = "Hello, world!";

    assert_eq!("world", &s[7..12]);
}