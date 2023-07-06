fn push(&mut self, delegate: Delegate<T>) {
    unsafe {
        std::ptr::write((*self.buffer).as_mut_ptr().add(self.len), delegate);
        self.len += 1;
    }
}
// https://github.com/makepad/makepad/blob/17eef68bd1347eca1b920390642f9f44f25e4a6f/libs/windows/src/core/event.rs#L156