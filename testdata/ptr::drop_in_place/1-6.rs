unsafe fn drop_value<T>(p: *mut ()) {
    ptr::drop_in_place(p.cast::<T>());
}
// https://github.com/datenlord/datenlord/blob/c8b5e25021ab36d9faee8ecfacf7b7da0cf9f250/src/async_fuse/proactor/small_box.rs#L23