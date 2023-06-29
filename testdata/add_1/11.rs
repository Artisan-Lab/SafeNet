pub(crate) fn read_vec_meta<T: IoVecBuf>(buf: &T) -> IoVecMeta {
    {
        let ptr = buf.read_iovec_ptr();
        let iovec_len = buf.read_iovec_len();

        let mut data = Vec::with_capacity(iovec_len);
        let mut len = 0;
        for i in 0..iovec_len {
            let iovec = unsafe { *ptr.add(i) };
            data.push(iovec);
            len += iovec.iov_len;
        }
        IoVecMeta {
            data,
            offset: 0,
            len,
        }
    }
}
// https://github.com/bytedance/monoio/blob/9c3592cbb15cda6091c98736e3db9dfd6a38c9ac/monoio/src/buf/vec_wrapper.rs#L25