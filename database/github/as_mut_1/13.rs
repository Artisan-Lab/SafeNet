fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
    if self.value.is_none() {
        return Poll::Ready(());
    }

    STORE.with(|cell| {
        let ptr = cell.get() as *mut Option<T>;
        let option_ref = unsafe { ptr.as_mut() }.expect("invalid usage");

        if option_ref.is_none() {
            *option_ref = self.value.take();
        }

        Poll::Pending
    })
}

// https://github.com/tokio-rs/async-stream/blob/3bad70289ea0e2872ae0031c24ab7e19b5ea6b98/async-stream/src/yielder.rs#L63