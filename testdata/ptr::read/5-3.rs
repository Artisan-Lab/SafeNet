pub fn into_boxed_slice(mut self) -> Box<[T], A> {
    unsafe {
        self.shrink_to_fit();
        let me = ManuallyDrop::new(self);
        let buf = ptr::read(&me.buf);
        let len = me.len();
        buf.into_box(len).assume_init()
    }
}
// https://github.com/Rust-for-Linux/linux/blob/c8d1ae2cbe240789ad402c71fce78a7ea1ebdea5/rust/alloc/vec/mod.rs#L1245