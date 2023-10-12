pub fn new_with<F>(py: Python<'_>, len: usize, init: F) -> PyResult<&PyBytes>
    where
        F: FnOnce(&mut [u8]) -> PyResult<()>,
    {
        unsafe {
            let pyptr = ffi::PyBytes_FromStringAndSize(std::ptr::null(), len as ffi::Py_ssize_t);
            // Check for an allocation error and return it
            let pypybytes: Py<PyBytes> = Py::from_owned_ptr_or_err(py, pyptr)?;
            let buffer = ffi::PyBytes_AsString(pyptr) as *mut u8;
            debug_assert!(!buffer.is_null());
            // Zero-initialise the uninitialised bytestring
            std::ptr::write_bytes(buffer, 0u8, len);
            // (Further) Initialise the bytestring in init
            // If init returns an Err, pypybytearray will automatically deallocate the buffer
            init(std::slice::from_raw_parts_mut(buffer, len)).map(|_| pypybytes.into_ref(py))
        }
    }
    // https://github.com/PyO3/pyo3/blob/54ab9090be944d72acbb78445c043b0c0e12a2ae/src/types/bytes.rs#L64