pub fn into_boxed_os_str(self) -> Box<OsStr> {
    let rw = Box::into_raw(self.inner.into_box()) as *mut OsStr;
    unsafe { Box::from_raw(rw) }
}

// https://github.com/esp-rs/rust/blob/ed3726ba7aea45731260ec8f75f05fc60c2b0f22/library/std/src/ffi/os_str.rs#L466