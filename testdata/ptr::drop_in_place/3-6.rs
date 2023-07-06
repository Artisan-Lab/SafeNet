pub fn release(self, _py: Python<'_>) {
    // First move self into a ManuallyDrop, so that PyBuffer::drop will
    // never be called. (It would acquire the GIL and call PyBuffer_Release
    // again.)
    let mut mdself = mem::ManuallyDrop::new(self);
    unsafe {
        // Next, make the actual PyBuffer_Release call.
        ffi::PyBuffer_Release(&mut *mdself.0);

        // Finally, drop the contained Pin<Box<_>> in place, to free the
        // Box memory.
        let inner: *mut Pin<Box<ffi::Py_buffer>> = &mut mdself.0;
        ptr::drop_in_place(inner);
    }
}
// https://github.com/PyO3/pyo3/blob/54ab9090be944d72acbb78445c043b0c0e12a2ae/src/buffer.rs#L635