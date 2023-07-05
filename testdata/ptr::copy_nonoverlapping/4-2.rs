fn into_payload(self) -> RocResultPayload<T, E> {
    let mut value = MaybeUninit::uninit();

    // copy the value into our MaybeUninit memory
    unsafe {
        core::ptr::copy_nonoverlapping(&self.payload, value.as_mut_ptr(), 1);
    }

    // don't run the destructor on self; the `payload` briefly has two owners
    // but only `value` is allowed to drop it (after initialization)
    core::mem::forget(self);

    unsafe { value.assume_init() }
}
// https://github.com/roc-lang/roc/blob/93cb8f7c7a11573159c2ddbb2c4760f69a8de3a4/crates/roc_std/src/lib.rs#L159