pub fn into_vec(mut self) -> Vec<T> {
    let len = core::mem::replace(&mut self.len, 0);
    let storage = core::mem::replace(&mut self.storage, Box::new([]));
    let slice = Box::leak(storage);
    debug_assert!(len <= slice.len());
    // SAFETY: valid elements.
    unsafe { Vec::from_raw_parts(slice.as_mut_ptr() as *mut T, len, slice.len()) }
}

// https://github.com/gimli-rs/gimli/blob/c42dec01df24d9a5cdbdd8ca1aea555242cc64e9/src/read/util.rs#L184