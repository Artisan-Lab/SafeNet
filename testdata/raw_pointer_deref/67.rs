fn read_raw_bytes(&mut self, s: &mut [MaybeUninit<u8>]) -> Result<(), Self::Error> {
    
        for c in s.iter_mut() {
            let h = self.read_u8()?;
            unsafe { *c.as_mut_ptr() = h };
            
        }
        Ok(())
    }