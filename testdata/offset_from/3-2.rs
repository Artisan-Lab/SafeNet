#[inline]
pub unsafe fn offset_from<U>(self, origin: BitPtr<M, U, O>) -> isize
    where U: BitStore<Mem = T::Mem> {
    self.get_addr()
        .cast::<T::Mem>()
        .offset_from(origin.get_addr().cast::<T::Mem>())
        .wrapping_mul(mem::bits_of::<T::Mem>() as isize)
        .wrapping_add(self.bit.into_inner() as isize)
        .wrapping_sub(origin.bit.into_inner() as isize)
}