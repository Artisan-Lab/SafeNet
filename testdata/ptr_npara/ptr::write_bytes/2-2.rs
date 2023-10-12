fn clear_all(&mut self) {
    unsafe {
        ptr::write_bytes(self.tabs.as_mut_ptr(), 0, self.tabs.len());
    }
}
// https://github.com/alacritty/alacritty/blob/edf4df66c9afcb9593eaeaaf34d5c80c66925adc/alacritty_terminal/src/term/mod.rs#L2017