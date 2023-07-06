pub fn finish(&mut self) -> NonNull<PyObject> {
    unsafe {
        std::ptr::write(self.buffer_ptr(), 0);
        (*self.bytes.cast::<PyVarObject>()).ob_size = self.len as Py_ssize_t;
        self.resize(self.len);
        NonNull::new_unchecked(self.bytes as *mut PyObject)
    }
}
// https://github.com/ijl/orjson/blob/314dc9da5b2d2ae515ec25e0fc5b503a4059b472/src/serialize/writer.rs#L31