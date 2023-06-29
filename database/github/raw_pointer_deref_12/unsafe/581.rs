unsafe fn deref_ptr(ptr: *const i32) -> i32 {
    if !ptr.is_null() {
        *ptr
    } else {
        panic!("Null pointer!");
    }
}
