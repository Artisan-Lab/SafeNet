fn read_bytes(&mut self, b: &mut [u8], offset: usize, len: usize) -> Result<()> {
    unsafe {
        let ptr = b.as_mut_ptr().add(offset);
        for i in 0..len {
            *ptr.add(i) = self.read_byte()?;
        }
    }

    Ok(())
}

/*
https://github.com/zhihu/rucene/blob/5b55f842c2bb03beb96898d520e880c180c91adf/src/core/util/fst/bytes_store.rs#L510
*/