fn next(&mut self) -> Option<T> {
    // Coerce the pointer early, so we don't keep the
    // reference that's about to be invalidated.
    let ptr: *const T = self.iter.next()?;
    Some(unsafe { ptr::read(ptr) })
}

// https://github.com/pexip/os-rust-rayon/blob/e5b81c8449796ca8cd69d28288c1766709d3be0c/src/vec.rs#L247