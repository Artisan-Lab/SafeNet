impl<'a> Iterator for SplitWhitespaceIndices<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.inner
            .next()
            .map(|str| unsafe { str.as_ptr().offset_from(self.str.as_ptr()) as usize })
    }
}
//%https://github.com/osa1/tiny/blob/f689c1b6ba1a9e9a220e675b14a54eb2d12d352d/crates/tiny/src/utils.rs#L16