pub unsafe fn assume_init(this: Self) -> Box<T, A> {
    let (ptr, alloc) = Self::into_raw_with_alloc(this);

    Box::from_raw_in(ptr.cast(), alloc)
}