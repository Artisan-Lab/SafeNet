pub fn peek(&self) -> Cow<'a, TokenReference<'a>> {
    if self.index >= self.len {
        panic!("peek failed, when there should always be an eof");
    }

    let result = unsafe {
        &*self
            .tokens
            .add(self.index)
            .as_ref()
            .expect("couldn't peek, no eof?")
    };

    Cow::Owned(result.to_owned())
}