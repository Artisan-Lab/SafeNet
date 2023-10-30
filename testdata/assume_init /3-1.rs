pub fn poll_recv(&self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
    let n = ready!(self.io.registration().poll_read_io(cx, || {
        // Safety: will not read the maybe uinitialized bytes.
        let b = unsafe {
            &mut *(buf.unfilled_mut() as *mut [std::mem::MaybeUninit<u8>] as *mut [u8])
        };

        self.io.recv(b)
    }))?;

    // Safety: We trust `recv` to have filled up `n` bytes in the buffer.
    unsafe {
        buf.assume_init(n);
    }
    buf.advance(n);
    Poll::Ready(Ok(()))
}