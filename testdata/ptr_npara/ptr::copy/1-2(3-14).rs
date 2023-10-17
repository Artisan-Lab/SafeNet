pub unsafe fn read(&self, dest: *mut u8, offset: usize, len: usize) -> Result {
    // TODO: For now this only works on the first page.
    let end = offset.checked_add(len).ok_or(EINVAL)?;
    if end > PAGE_SIZE {
        return Err(EINVAL);
    }

    let mapping = self.kmap(0).ok_or(EINVAL)?;
    unsafe { ptr::copy((mapping.ptr as *mut u8).add(offset), dest, len) };
    Ok(())
}
// https://github.com/fl0rek/linux/blob/5073cfdbeb886c633f4e2cf3b2c10f6791cae54e/rust/kernel/pages.rs#L78

// fn read(&self, dest: *mut u8, offset: usize, len: usize) -> Result {
//     let end = offset.checked_add(len).ok_or(EINVAL)?;
//     if end > PAGE_SIZE {
//         return Err(EINVAL);
//     }

//     let mapping = self.kmap(0).ok_or(EINVAL)?;
//     unsafe {
//         ptr::copy((mapping.ptr as *mut u8).add(offset), dest, len);
//     }
//     Ok(())
// }
