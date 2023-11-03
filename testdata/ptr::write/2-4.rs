pub fn replace<T, R>(v: &mut T, change: impl FnOnce(T) -> (T, R)) -> R {
    struct PanicGuard;
    impl Drop for PanicGuard {
        fn drop(&mut self) {
            intrinsics::abort()
        }
    }
    let guard = PanicGuard;
    let value = unsafe { ptr::read(v) };
    let (new_value, ret) = change(value);
    unsafe {
        ptr::write(v, new_value);
    }
    mem::forget(guard);
    ret
}

// https://github.com/glaubitz/rust/blob/1f5768bc67ecb025342770e14e03699699965706/library/alloc/src/collections/btree/mem.rs#L31