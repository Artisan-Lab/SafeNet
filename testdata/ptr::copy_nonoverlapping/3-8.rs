macro_rules! peek_u {
    ($b:expr, $ty:ty, $len:expr) => {{
        let len = $len;
        let src = &$b.buf[$b.off..];

        if src.len() < len {
            return Err(BufferTooShortError);
        }

        let mut out: $ty = 0;
        unsafe {
            let dst = &mut out as *mut $ty as *mut u8;
            let off = (mem::size_of::<$ty>() - len) as isize;

            ptr::copy_nonoverlapping(src.as_ptr(), dst.offset(off), len);
        };

        Ok(<$ty>::from_be(out))
    }};
}
// https://github.com/cloudflare/quiche/blob/e9b6670037e7b28b8626a5203144bf89c33cf166/octets/src/lib.rs#L69