pub unsafe extern "C" fn slint_callback_drop(handle: *mut CallbackOpaque) {
    core::ptr::drop_in_place(handle as *mut Callback<()>);
}
// https://github.com/slint-ui/slint/blob/9afadf50be58522f1a5b99c9b271211071610d5a/internal/core/callbacks.rs#L148