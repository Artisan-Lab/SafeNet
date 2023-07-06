#[inline(always)]
#[unstable(feature = "pointer_byte_offsets", issue = "96283")]
#[rustc_const_unstable(feature = "const_pointer_byte_offsets", issue = "96283")]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
pub const unsafe fn byte_offset_from<U: ?Sized>(self, origin: *const U) -> isize {
    // SAFETY: the caller must uphold the safety contract for `offset_from`.
    unsafe { self.cast::<u8>().offset_from(origin.cast::<u8>()) }
}
//https://github.com/esp-rs/rust/blob/ed3726ba7aea45731260ec8f75f05fc60c2b0f22/library/core/src/ptr/const_ptr.rs#L711