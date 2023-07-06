fn poll_read(
    &mut self,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<io::Result<()>> {
    ready!(self.poll_io(cx, Flow::Read))?;

    if self.rd_state == State::StreamOpen {
      let buf_slice =
        unsafe { &mut *(buf.unfilled_mut() as *mut [_] as *mut [u8]) };
      let bytes_read = self.tls.read(buf_slice)?;
      assert_ne!(bytes_read, 0);
      unsafe { buf.assume_init(bytes_read) };
      buf.advance(bytes_read);
    }

    Poll::Ready(Ok(()))
  }

  // https://github.com/leizongmin/deno/blob/5065c7bcd9973056b9b0d9df71d139da83596acc/ext/net/ops_tls.rs#L401