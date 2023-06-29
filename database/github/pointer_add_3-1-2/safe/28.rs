pub fn peek(&self) -> Cow<'a, TokenReference<'a>> {
    if self.index >= self.len {
        panic!("peek failed, when there should always be an eof");
    }

    let result = { self.tokens.get(self.index).expect("couldn't peek, no eof?") };

    Cow::Owned(result.to_owned())
}