fn from(v: Vec<NonZeroU8>) -> CString {
    unsafe {
        // Transmute `Vec<NonZeroU8>` to `Vec<u8>`.
        let v: Vec<u8> = {
            let (ptr, len, cap): (*mut NonZeroU8, _, _) = Vec::into_raw_parts(v);
            Vec::from_raw_parts(ptr.cast::<u8>(), len, cap)
        };
        Self::_from_vec_unchecked(v)
    }
}
// https://github.com/rust-lang/rust/blob/52d8c490a3aabe65cdd9f2d3aed95034dd5dbad7/library/alloc/src/ffi/c_str.rs#L810
