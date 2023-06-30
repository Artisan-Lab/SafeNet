pub fn copy_into(data: &[u8], size: usize, offset: usize) -> Self {
    let data = &data[..core::cmp::min(size, data.len())];

    // Allocate a buffer large enough for offset + size.
    let count = (offset + size + Page::SIZE - 1) / Page::SIZE;
    let mut buf = alloc::vec::Vec::with_capacity(count);
    let bytes: &mut [u8] = unsafe {
        buf.set_len(count);
        buf.align_to_mut().1
    };

    // Segment the regions.
    let (prefix, bytes) = bytes.split_at_mut(offset);
    let (bytes, suffix) = bytes.split_at_mut(data.len());

    // Copy and zero.
    prefix.fill(0);
    bytes.copy_from_slice(data);
    suffix.fill(0);

    Self(buf)
}

// https://github.com/enarx/primordial/blob/f2d4136d094811ed6d88041297fb56f96e29c2ed/src/pages.rs#L42