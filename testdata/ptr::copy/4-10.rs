pub fn remove(&mut self, idx: usize) -> char {
    let ch = match self[idx..].chars().next() {
        Some(ch) => ch,
        None => panic!("cannot remove a char from the end of a string"),
    };

    let next = idx + ch.len_utf8();
    let len = self.len();
    unsafe {
        ptr::copy(self.vec.as_ptr().add(next), self.vec.as_mut_ptr().add(idx), len - next);
        self.vec.set_len(len - (next - idx));
    }
    ch
}

// https://github.com/esp-rs/rust/blob/edc37d22db1cc1b112b91addeb1f79951c58e661/library/alloc/src/string.rs#L1344