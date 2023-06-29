let chars = "a楽".chars().enumerate();

let mut offset = 0;
for (i, c) in chars {
    assert_eq!(i, offset);
    offset += c.len_utf8();
}

assert_eq!(offset, 4);
