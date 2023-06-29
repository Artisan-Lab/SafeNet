#![feature(char_indices_offset)]
let mut chars = "a楽".char_indices();

assert_eq!(chars.offset(), 0);
assert_eq!(chars.next(), Some((0, 'a')));

assert_eq!(chars.offset(), 1);
assert_eq!(chars.next(), Some((1, '楽')));

assert_eq!(chars.offset(), 4);
assert_eq!(chars.next(), None);

//不稳定， alternative implementation using chars() and enumerate() iterators