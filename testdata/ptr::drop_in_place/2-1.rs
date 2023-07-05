fn drop(&mut self) {
    // Safety: `0` is always initialized.
    unsafe { core::ptr::drop_in_place(self.0.as_mut_ptr()) };
}
// https://github.com/tokio-rs/tokio/blob/0d382faa4e557d0f6e281485cfb14a4934461c41/tokio-util/src/util/maybe_dangling.rs#L33