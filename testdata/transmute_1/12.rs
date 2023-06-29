unsafe fn slice_assume_init_mut<T>(s: &mut [MaybeUninit<T>]) -> &mut [T] {
    std::mem::transmute(s)
}
