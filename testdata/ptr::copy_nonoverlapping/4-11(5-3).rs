pub fn format_decimal(v: i128, scale: usize, trim_zeros: bool) -> FormatBuffer {
    const ZEROS: [u8; BUF_LEN] = [b'0'; BUF_LEN];

    let mut buf = FormatBuffer::new();
    let factor = POW10[scale]; //10_i128.pow(scale as _);
    let (div, rem) = (v / factor, v.abs() % factor);

    unsafe {
        let mut ptr = buf.data.as_mut_ptr();
        if div == 0 && v < 0 {
            *ptr = b'-';
            ptr = ptr.add(1);
            buf.len = 1;
        }
        let n_whole = itoap::write_to_ptr(ptr, div);
        buf.len += n_whole;
        if rem != 0 {
            ptr = ptr.add(n_whole);
            *ptr = b'.';
            ptr = ptr.add(1);
            let mut frac_buf = [0_u8; BUF_LEN];
            let n_frac = itoap::write_to_ptr(frac_buf.as_mut_ptr(), rem);
            ptr::copy_nonoverlapping(ZEROS.as_ptr(), ptr, scale - n_frac);
            ptr = ptr.add(scale - n_frac);
            ptr::copy_nonoverlapping(frac_buf.as_mut_ptr(), ptr, n_frac);
            buf.len += 1 + scale;
            if trim_zeros {
                ptr = ptr.add(n_frac - 1);
                while *ptr == b'0' {
                    ptr = ptr.sub(1);
                    buf.len -= 1;
                }
            }
        }
    }

    buf
}
// https://github.com/pola-rs/polars/blob/86aafa82b3cb83e7ca9f8f9d4672f39a38bee0ab/crates/polars-core/src/fmt.rs#L1055