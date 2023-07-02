fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let self_: &mut PopFuture = self.get_mut();
        let mut bytes: [MaybeUninit<u8>; POP_SIZE] = MaybeUninit::uninit_array();
        match self_.socket.borrow().recv_from(&mut bytes[..]) {
            Ok((nbytes, socketaddr)) => {
                trace!("data received ({:?}/{:?} bytes)", nbytes, POP_SIZE);
                unsafe {
                    let bytes_recv: [u8; POP_SIZE] = transmute::<[MaybeUninit<u8>; POP_SIZE], [u8; POP_SIZE]>(bytes);
                    let buf: DemiBuffer = DemiBuffer::from_slice(&bytes_recv[0..nbytes])?;
                    Poll::Ready(Ok((socketaddr.as_socket_ipv4(), buf)))
                }
            },
        }
}
