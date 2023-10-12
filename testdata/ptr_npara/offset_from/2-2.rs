fn validate_layout<T: InfoInternal + ?Sized>(info: &T, name: &[Char16]) {
    // Check the hardcoded struct alignment.
    assert_eq!(mem::align_of_val(info), T::alignment());
    // Check the hardcoded name slice offset.
    assert_eq!(
        unsafe { (name.as_ptr() as *const u8).offset_from(info as *const _ as *const u8) },
        T::name_offset() as isize
    );
}
//https://github.com/rust-osdev/uefi-rs/blob/6b9a4bdb48036d37ab037924a4392d7f7e823db3/uefi/src/proto/media/file/info.rs#L424