fn next(&mut self) -> Option<T> {
    self.iter.next().map(|elt| unsafe { ptr::read(elt as *const _) })
}
// https://github.com/Rust-for-Linux/linux/blob/c8d1ae2cbe240789ad402c71fce78a7ea1ebdea5/rust/alloc/vec/drain.rs#L161