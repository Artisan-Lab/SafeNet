fn inc_ref<T: IsObject>(ptr: NonNull<T>) {
    use std::ptr::NonNull;
    unsafe { ptr.as_ref().as_object().inc_ref() }
}
/*
https://github.com/szha/tvm/blob/f34e3a88a01a347e7cee02c7085b837baef1fb72/rust/tvm-rt/src/object/object_ptr.rs#L22
*/
