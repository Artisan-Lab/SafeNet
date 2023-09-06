unsafe fn from_boxed_utf8_unchecked(v: Box<[u8]>) -> Box<str> {
    unsafe { Box::from_raw(Box::into_raw(v) as *mut str) }
}

// https://github.com/bjorn3/rust/blob/2f896da247e0ee8f0bef7cd7c54cfbea255b9f68/library/alloc/src/str.rs#L617