unsafe fn transize<T, const N: usize>(
    f: unsafe fn(T, T) -> T,
    bytes: Simd<u8, N>,
    idxs: Simd<u8, N>,
) -> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let idxs = zeroing_idxs(idxs);
    // SAFETY: Same obligation to use this function as to use mem::transmute_copy.
    unsafe { mem::transmute_copy(&f(mem::transmute_copy(&bytes), mem::transmute_copy(&idxs))) }
}

// https://github.com/bjorn3/rust/blob/2f896da247e0ee8f0bef7cd7c54cfbea255b9f68/library/portable-simd/crates/core_simd/src/swizzle_dyn.rs#L123
