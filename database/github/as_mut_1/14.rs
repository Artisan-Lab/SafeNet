pub fn get_mut_checked<'a, T>(ptr: *mut T) -> Result<&'a mut T, String> {
    match check_ptr_is_non_null_and_aligned(ptr) {
        Ok(()) => unsafe {
            ptr.as_mut()
                .ok_or_else(|| "Error while converting to mut reference".into())
        },
        Err(e) => Err(e),
    }
}