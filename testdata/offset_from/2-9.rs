impl<'a> ops::Sub<RefPosition<'a>> for RefPosition<'a> {
    type Output = usize;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(self.0 >= rhs.0, "Underflow");
        // Note Rust has backwards naming here. The "origin" is self, not the param; the rhs is the offset value.
        unsafe { self.ptr().offset_from(rhs.ptr()) as usize }
    }
}
//https://github.com/ridiculousfish/regress/blob/72df502537a5ff9f5a7c53883c936f933d205b34/src/position.rs#L144