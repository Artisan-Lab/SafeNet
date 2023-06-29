pub fn from_current(&mut self) {
    unsafe {
        T::from_current(self);

        let ptr = Arc::into_raw(self.me.upgrade().unwrap());
        Arc::decrement_strong_count(ptr);
        Arc::from_raw(ptr);
    }
    self.state = State::Ready;
}
/*
https://github.com/Samsung/islet/blob/c34b1d105be4e077cc67e89094739a87a28a2d85/rmm/monitor/src/realm/vcpu.rs#L59
*/