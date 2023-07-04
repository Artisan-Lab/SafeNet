fn initialize(&self) -> *mut T {
    let new_ptr = Box::into_raw(T::init());
    match self.ptr.compare_exchange(null_mut(), new_ptr, AcqRel, Acquire) {
        Ok(_) => new_ptr,
        Err(ptr) => {
            // Lost the race to another thread.
            // Drop the box we created, and use the one from the other thread instead.
            T::cancel_init(unsafe { Box::from_raw(new_ptr) });
            ptr
        }
    }
}
// https://github.com/bjorn3/rust/blob/2f896da247e0ee8f0bef7cd7c54cfbea255b9f68/library/std/src/sys_common/lazy_box.rs#L61