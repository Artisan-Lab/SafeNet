impl Deref for CharBuffer {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { str::from_utf8_unchecked(self.buf.get_unchecked(..self.len)) }
    }
}
/*
https://github.com/lapce/lapce/blob/6c85e94e1d21ad2dfc42b9756f2806b5e67d6160/lapce-core/src/char_buffer.rs#L545
*/