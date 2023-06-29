impl AsMut<[u8]> for Inner {
    fn as_mut(&mut self) -> &mut [u8] {
        
        self.buf_mut()
    }
}