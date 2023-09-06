pub fn header(&self) -> &Header {
    use std::ptr::NonNull;
    unsafe { self.ptr.as_ref() }
}
/*
https://github.com/passcod/tokio/blob/f2a06bff1be147a72d40cb01d8bb621fbdc242fc/tokio/src/runtime/task/raw.rs#L4
*/