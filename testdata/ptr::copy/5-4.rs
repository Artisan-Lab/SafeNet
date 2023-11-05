unsafe fn nonnull_filter_impl<T, I>(values: &[T], mut mask_chunks: I, filter_count: usize) -> Vec<T>
where
    T: NativeType + Simd,
    I: BitChunkIterExact<u64>,
{
    let mut chunks = values.chunks_exact(64);
    let mut new = Vec::<T>::with_capacity(filter_count);
    let mut dst = new.as_mut_ptr();

    chunks
        .by_ref()
        .zip(mask_chunks.by_ref())
        .for_each(|(chunk, mask_chunk)| {
            let ones = mask_chunk.count_ones();
            let leading_ones = get_leading_ones(mask_chunk);

            if ones == leading_ones {
                let size = leading_ones as usize;
                unsafe {
                    std::ptr::copy(chunk.as_ptr(), dst, size);
                    dst = dst.add(size);
                }
                return;
            }

            let ones_iter = BitChunkOnes::from_known_count(mask_chunk, ones as usize);
            for pos in ones_iter {
                dst.write(*chunk.get_unchecked(pos));
                dst = dst.add(1);
            }
        });

    chunks
        .remainder()
        .iter()
        .zip(mask_chunks.remainder_iter())
        .for_each(|(value, b)| {
            if b {
                unsafe {
                    dst.write(*value);
                    dst = dst.add(1);
                };
            }
        });

    unsafe { new.set_len(filter_count) };
    new
}

// https://github.com/pola-rs/polars/blob/05d9eb20461c44611c4661770f68a50a0072a00a/crates/nano-arrow/src/compute/filter.rs#L46