#![allow(unused)]
use std::str;

fn main() {
    let sparkle_heart = vec![240, 159, 146, 150];

    let sparkle_heart = unsafe {
        str::from_utf8_unchecked(&sparkle_heart)
    };
}