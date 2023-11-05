pub fn read_at(&self, buf: &mut [u8], offset: usize) -> Option<usize> {
    self.sub_region(offset, buf.len()).map(|sub_region| {
        let mut bytes = 0;
        let mut buf_ptr = buf.as_mut_ptr();

        sub_region.into_iter().for_each(|iov| {
            let src = iov.iov_base.cast::<u8>();
            // SAFETY:
            // The call to `copy_nonoverlapping` is safe because:
            // 1. `iov` is a an iovec describing a segment inside `Self`. `IoVecSubregion` has
            //    performed all necessary bound checks.
            // 2. `buf_ptr` is a pointer inside the memory of `buf`
            // 3. Both pointers point to `u8` elements, so they're always aligned.
            // 4. The memory regions these pointers point to are not overlapping. `src` points
            //    to guest physical memory and `buf_ptr` to Firecracker-owned memory.
            //
            // `buf_ptr.add()` is safe because `IoVecSubregion` gives us `iovec` structs that
            // their size adds up to `buf.len()`.
            unsafe {
                std::ptr::copy_nonoverlapping(src, buf_ptr, iov.iov_len);
                buf_ptr = buf_ptr.add(iov.iov_len);
            }
            bytes += iov.iov_len;
        });

        bytes
    })
}
// https://github.com/firecracker-microvm/firecracker/blob/cd2eddeb50b2a702cce60a8116fc8617eee206ac/src/vmm/src/devices/virtio/iovec.rs#L204