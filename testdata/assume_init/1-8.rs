pub fn take(&mut self) -> Option<T> {
    if self.is_initialized() {
        self.once = Once::new();
        // SAFETY: `self.value` is initialized and contains a valid `T`.
        // `self.once` is reset, so `is_initialized()` will be false again
        // which prevents the value from being read twice.
        unsafe { Some((&mut *self.value.get()).assume_init_read()) }
    } else {
        None
    }
}

// https://github.com/jplatte/rust/blob/f5b8f44e5d5dee0f60cec1729b5a107659779d94/library/std/src/sync/once_lock.rs#L274