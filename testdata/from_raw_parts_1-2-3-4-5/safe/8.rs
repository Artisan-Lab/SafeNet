impl AsRef<[u8]> for Inner {
    fn as_ref(&self) -> &[u8] {
        
        self.buf()
    }
}