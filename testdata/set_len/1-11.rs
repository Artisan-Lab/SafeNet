/*
https://github.com/aknuds1/tokio/blob/1eefbc250ad49d9086bdb2b8be4404bd445f63fb/tokio/src/io/util/read_to_end.rs#L84
*/
fn poll_read_to_end<R: AsyncRead + ?Sized>(
    buf: &mut Vec<u8>,
    read: Pin<&mut R>,
    cx: &mut Context<'_>,
) -> Poll<io::Result<usize>> {
    // This uses an adaptive system to extend the vector when it fills. We want to
    // avoid paying to allocate and zero a huge chunk of memory if the reader only
    // has 4 bytes while still making large reads if the reader does have a ton
    // of data to return. Simply tacking on an extra DEFAULT_BUF_SIZE space every
    // time is 4,500 times (!) slower than this if the reader has a very small
    // amount of data to return.
    reserve(buf, 32);

    let mut unused_capacity = ReadBuf::uninit(get_unused_capacity(buf));

    ready!(read.poll_read(cx, &mut unused_capacity))?;

    let n = unused_capacity.filled().len();
    let new_len = buf.len() + n;

    // This should no longer even be possible in safe Rust. An implementor
    // would need to have unsafely *replaced* the buffer inside `ReadBuf`,
    // which... yolo?
    assert!(new_len <= buf.capacity());
    unsafe {
        buf.set_len(new_len);
    }
    Poll::Ready(Ok(n))
}