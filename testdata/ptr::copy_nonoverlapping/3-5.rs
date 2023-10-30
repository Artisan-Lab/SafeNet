fn parse_u64_into<const N: usize>(mut n: u64, buf: &mut [MaybeUninit<u8>; N], curr: &mut usize) {
    let buf_ptr = MaybeUninit::slice_as_mut_ptr(buf);
    let lut_ptr = DEC_DIGITS_LUT.as_ptr();
    assert!(*curr > 19);

    // SAFETY:
    // Writes at most 19 characters into the buffer. Guaranteed that any ptr into LUT is at most
    // 198, so will never OOB. There is a check above that there are at least 19 characters
    // remaining.
    unsafe {
        if n >= 1e16 as u64 {
            let to_parse = n % 1e16 as u64;
            n /= 1e16 as u64;

            // Some of these are nops but it looks more elegant this way.
            let d1 = ((to_parse / 1e14 as u64) % 100) << 1;
            let d2 = ((to_parse / 1e12 as u64) % 100) << 1;
            let d3 = ((to_parse / 1e10 as u64) % 100) << 1;
            let d4 = ((to_parse / 1e8 as u64) % 100) << 1;
            let d5 = ((to_parse / 1e6 as u64) % 100) << 1;
            let d6 = ((to_parse / 1e4 as u64) % 100) << 1;
            let d7 = ((to_parse / 1e2 as u64) % 100) << 1;
            let d8 = ((to_parse / 1e0 as u64) % 100) << 1;

            *curr -= 16;

            ptr::copy_nonoverlapping(lut_ptr.add(d1 as usize), buf_ptr.add(*curr + 0), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d2 as usize), buf_ptr.add(*curr + 2), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d3 as usize), buf_ptr.add(*curr + 4), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d4 as usize), buf_ptr.add(*curr + 6), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d5 as usize), buf_ptr.add(*curr + 8), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d6 as usize), buf_ptr.add(*curr + 10), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d7 as usize), buf_ptr.add(*curr + 12), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d8 as usize), buf_ptr.add(*curr + 14), 2);
        }
        if n >= 1e8 as u64 {
            let to_parse = n % 1e8 as u64;
            n /= 1e8 as u64;

            // Some of these are nops but it looks more elegant this way.
            let d1 = ((to_parse / 1e6 as u64) % 100) << 1;
            let d2 = ((to_parse / 1e4 as u64) % 100) << 1;
            let d3 = ((to_parse / 1e2 as u64) % 100) << 1;
            let d4 = ((to_parse / 1e0 as u64) % 100) << 1;
            *curr -= 8;

            ptr::copy_nonoverlapping(lut_ptr.add(d1 as usize), buf_ptr.add(*curr + 0), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d2 as usize), buf_ptr.add(*curr + 2), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d3 as usize), buf_ptr.add(*curr + 4), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d4 as usize), buf_ptr.add(*curr + 6), 2);
        }
        // `n` < 1e8 < (1 << 32)
        let mut n = n as u32;
        if n >= 1e4 as u32 {
            let to_parse = n % 1e4 as u32;
            n /= 1e4 as u32;

            let d1 = (to_parse / 100) << 1;
            let d2 = (to_parse % 100) << 1;
            *curr -= 4;

            ptr::copy_nonoverlapping(lut_ptr.add(d1 as usize), buf_ptr.add(*curr + 0), 2);
            ptr::copy_nonoverlapping(lut_ptr.add(d2 as usize), buf_ptr.add(*curr + 2), 2);
        }

        // `n` < 1e4 < (1 << 16)
        let mut n = n as u16;
        if n >= 100 {
            let d1 = (n % 100) << 1;
            n /= 100;
            *curr -= 2;
            ptr::copy_nonoverlapping(lut_ptr.add(d1 as usize), buf_ptr.add(*curr), 2);
        }

        // decode last 1 or 2 chars
        if n < 10 {
            *curr -= 1;
            *buf_ptr.add(*curr) = (n as u8) + b'0';
        } else {
            let d1 = n << 1;
            *curr -= 2;
            ptr::copy_nonoverlapping(lut_ptr.add(d1 as usize), buf_ptr.add(*curr), 2);
        }
    }
}
// https://github.com/bjorn3/rust/blob/2f896da247e0ee8f0bef7cd7c54cfbea255b9f68/library/core/src/fmt/num.rs#L517