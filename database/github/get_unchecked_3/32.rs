pub fn split_at_byte(&self, b: u8) -> (&RawStr, &RawStr) {
    if !b.is_ascii() {
        return (self, &self[0..0]);
    }

    match memchr::memchr(b, self.as_bytes()) {
        // SAFETY: `b` is a character boundary since it's ASCII, `i` is in
        // bounds in `self` (or else None), and i is at most len - 1, so i +
        // 1 is at most len.
        Some(i) => unsafe {
            let s = self.as_str();
            let start = s.get_unchecked(0..i);
            let end = s.get_unchecked((i + 1)..self.len());
            (start.into(), end.into())
        },
        None => (self, &self[0..0])
    }
}
/*
https://github.com/SergioBenitez/Rocket/blob/9a9cd76c0121f46765ff0df9ef81e36563a2a31f/core/http/src/raw_str.rs#L790
 */