pub fn write(&mut self, t: T) {
    if self.filled {
        // Drop the old before we overwrite it.
        unsafe { ptr::drop_in_place(self.data[self.write_at].as_mut_ptr()) }
    }
    self.data[self.write_at] = MaybeUninit::new(t);

    self.write_at += 1;
    if self.write_at == self.capacity() {
        self.write_at = 0;
        self.filled = true;
    }
}
// https://github.com/japaric/heapless/blob/644653bf3b831c6bb4963be2de24804acf5e5001/src/histbuf.rs#L131