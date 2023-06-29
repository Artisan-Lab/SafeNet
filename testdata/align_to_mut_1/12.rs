pub fn gen(&mut self, dst: &mut [u8]) {
    {
        let (_prefix, aligned, _suffix) = unsafe { dst.align_to_mut::<u128>() };
        debug_assert_eq!(_prefix.len(), 0);
        debug_assert_eq!(_suffix.len(), 0);
        for word in aligned.iter_mut() {
            *word = 0;
        }
    }
    self.xor_bytes(dst);
}

// https://github.com/trailofbits/reverie/blob/bd091087be79ea7590cffea4d6f762171d8e9d38/src/crypto/prg.rs#L29