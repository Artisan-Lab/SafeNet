fn copy_as<T>(&self, offset: usize) -> Result<T> {
    self.is_proper_length::<T>(offset)?;

    unsafe {
        let mut data = std::mem::MaybeUninit::zeroed().assume_init();
        std::ptr::copy_nonoverlapping(self[offset..].as_ptr(), &mut data as *mut T as *mut u8, std::mem::size_of::<T>());
        Ok(data)
    }
}
// https://github.com/microsoft/windows-rs/blob/7b424c2590ac21b47e8ac16b31460fd625e28122/crates/libs/metadata/src/file.rs#L488