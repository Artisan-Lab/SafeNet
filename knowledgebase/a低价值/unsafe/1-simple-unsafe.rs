/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_unchecked
    Purpose: Converts a vector of bytes to a String without checking that the string contains valid UTF-8.
    Replaceable? Yes
*/

#![allow(unused)]

fn main() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = unsafe {
        String::from_utf8_unchecked(sparkle_heart)
    };
    assert_eq!("💖", sparkle_heart);
}
