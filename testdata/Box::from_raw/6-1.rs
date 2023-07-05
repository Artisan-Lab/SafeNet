extern "system" fn child_exit_callback(ctx: *mut c_void, timed_out: BOOLEAN) {
    if timed_out != 0 {
        return;
    }

    let event_tx: Box<_> = unsafe { Box::from_raw(ctx as *mut Sender<ChildEvent>) };
    let _ = event_tx.send(ChildEvent::Exited);
}
// https://github.com/alacritty/alacritty/blob/edf4df66c9afcb9593eaeaaf34d5c80c66925adc/alacritty_terminal/src/tty/windows/child.rs#L21