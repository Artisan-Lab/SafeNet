
    fn read_raw_bytes_into(&mut self, s: &mut [u8]) -> Result<(), Self::Error> {
        for c in s.iter_mut() {

            *c = self.read_u8()?;
        }
        Ok(())
    }