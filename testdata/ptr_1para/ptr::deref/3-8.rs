pub unsafe fn get(&'static self, init: impl FnOnce() -> T) -> Option<&'static T> {
    // SAFETY: See the documentation for this method.
    let ptr = unsafe { self.os.get() as *mut Value<T> };
    if ptr.addr() > 1 {
        // SAFETY: the check ensured the pointer is safe (its destructor
        // is not running) + it is coming from a trusted source (self).
        if let Some(ref value) = unsafe { (*ptr).inner.get() } {
            return Some(value);
        }
    }
    // SAFETY: At this point we are sure we have no value and so
    // initializing (or trying to) is safe.
    unsafe { self.try_initialize(init) }
}
//https://github.com/bjorn3/rust/blob/2f896da247e0ee8f0bef7cd7c54cfbea255b9f68/library/std/src/sys/common/thread_local/os_local.rs#L126