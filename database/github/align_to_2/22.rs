fn rand_blob(rng: &mut RngType, len: usize) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::with_capacity(len);
    // SAFETY: sizeof `buf` equals to `len`.
    unsafe { buf.set_len(len) };
    let (prefix, shorts, suffix) = unsafe { buf.align_to_mut::<u64>() };
    prefix.fill_with(|| rng.gen());
    shorts.fill_with(|| rng.gen());
    suffix.fill_with(|| rng.gen());
    buf
}
// https://github.com/SunHao-0/healer/blob/2efbb44c7dfaaa6749cd1541949b26fd1d2143fa/healer_core/src/gen/buffer.rs#L36