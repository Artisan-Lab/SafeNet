pub fn copy_into(data: &[u8], size: usize, offset: usize) -> Self {
    let data = &data[..core::cmp::min(size, data.len())];

    // Allocate a buffer large enough for offset + size.
    let count = (offset + size + Page::SIZE - 1) / Page::SIZE;
    let mut buf = alloc::vec::Vec::with_capacity(count);
    let bytes: &mut [u8] = unsafe {
        buf.set_len(count);
        buf.align_to_mut().1
    };
}