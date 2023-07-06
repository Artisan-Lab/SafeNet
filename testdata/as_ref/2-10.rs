impl<T: AlwaysRefCounted> Deref for ARef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The type invariants guarantee that the object is valid.
        unsafe { self.ptr.as_ref() }
    }
}
//https://github.com/beagleboard/linux/blob/98be618ad03010b1173fc3c35f6cbb4447ee2b07/rust/kernel/types.rs#L370