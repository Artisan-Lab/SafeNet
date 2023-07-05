/*
https://github.com/rayon-rs/rayon/blob/6a9deff7a99e7c7b03bab694bbe8154d6fe2de10/src/vec.rs#L153
*/
  fn drop(&mut self) {
        let Range { start, end } = self.range;
        if self.vec.len() == self.orig_len {
            // We must not have produced, so just call a normal drain to remove the items.
            self.vec.drain(start..end);
        } else if start == end {
            // Empty range, so just restore the length to its original state
            unsafe {
                self.vec.set_len(self.orig_len);
            }
        } else if end < self.orig_len {
            // The producer was responsible for consuming the drained items.
            // Move the tail items to their new place, then set the length to include them.
            unsafe {
                let ptr = self.vec.as_mut_ptr().add(start);
                let tail_ptr = self.vec.as_ptr().add(end);
                let tail_len = self.orig_len - end;
                ptr::copy(tail_ptr, ptr, tail_len);
                self.vec.set_len(start + tail_len);
            }
        }
    }