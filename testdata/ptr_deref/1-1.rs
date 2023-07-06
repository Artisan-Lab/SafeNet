unsafe fn get_and_increment<T>(ptr: &mut *mut T) -> *mut T {
    let old = *ptr;
    *ptr = unsafe { ptr.offset(1) };
    old
}
// https://github.com/STMicroelectronics/linux/blob/d33b43a4dcc4ae3cd178793c139756af77e42bde/rust/alloc/slice.rs#L1040