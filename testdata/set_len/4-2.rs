/*
https://github.com/nielx/rust/blob/e8838e7fbae25491fc9910c7cb105fb7b7e12623/library/alloc/src/string.rs#L1466
*/
 fn drop(&mut self) {
                let new_len = self.idx - self.del_bytes;
                debug_assert!(new_len <= self.s.len());
                unsafe { self.s.vec.set_len(new_len) };
            }