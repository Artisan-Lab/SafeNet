fn convert_while_ascii(b: &[u8], convert: fn(&u8) -> u8, out: &mut Vec<u8>) {
    unsafe {
        out.set_len(0);
        out.reserve(b.len());
    }

    const USIZE_SIZE: usize = std::mem::size_of::<usize>();
    const MAGIC_UNROLL: usize = 2;
    const N: usize = USIZE_SIZE * MAGIC_UNROLL;
    const NONASCII_MASK: usize = usize::from_ne_bytes([0x80; USIZE_SIZE]);

    let mut i = 0;
    unsafe {
        while i + N <= b.len() {
            let in_chunk = b.get_unchecked(i..i + N);
            let out_chunk = out.spare_capacity_mut().get_unchecked_mut(i..i + N);

            let mut bits = 0;
            for j in 0..MAGIC_UNROLL {
                bits |= in_chunk.as_ptr().cast::<usize>().add(j).read_unaligned();
            }
            if bits & NONASCII_MASK != 0 {
                break;
            }
            for j in 0..N {
                let out = out_chunk.get_unchecked_mut(j);
                out.write(convert(in_chunk.get_unchecked(j)));
            }
            i += N;
        }
        out.set_len(i);
    }
}
/*
https://github.com/pola-rs/polars/blob/f94abbbf87034d37a6f9f7d2b856c327a5d79417/polars/polars-ops/src/chunked_array/strings/case.rs#L23
*/