fn new(slice: &[u8]) -> Self {
    let mut data = [0_u8; SZ];
    if slice.len() <= CUTOFF {
        data[SZ - 1] = (u8::try_from(slice.len()).unwrap() << 1) | 1;
        data[..slice.len()].copy_from_slice(slice);
    } else {
        let layout = Layout::from_size_align(
            slice.len() + size_of::<RemoteHeader>(),
            8,
        )
        .unwrap();

        let header = RemoteHeader { rc: 1.into(), len: slice.len() };

        unsafe {
            let ptr = alloc(layout);

            std::ptr::write(ptr as *mut RemoteHeader, header);
            std::ptr::copy_nonoverlapping(
                slice.as_ptr(),
                ptr.add(size_of::<RemoteHeader>()),
                slice.len(),
            );
            std::ptr::write_unaligned(data.as_mut_ptr() as _, ptr);
        }

        // assert that the bottom 3 bits are empty, as we expect
        // the buffer to always have an alignment of 8 (2 ^ 3).
        #[cfg(not(miri))]
        assert_eq!(data[SZ - 1] & 0b111, 0);
    }
    Self(data)
}
// https://github.com/spacejam/sled/blob/69294e59c718289ab3cb6bd03ac3b9e1e072a1e7/src/ivec.rs#L138