impl<T: PyClass> PyObjectInit<T> for PyClassInitializer<T> {
    unsafe fn into_new_object(
        self,
        py: Python<'_>,
        subtype: *mut PyTypeObject,
    ) -> PyResult<*mut ffi::PyObject> {
        /// Layout of a PyCell after base new has been called, but the contents have not yet been
        /// written.
        #[repr(C)]
        struct PartiallyInitializedPyCell<T: PyClass> {
            _ob_base: <T::BaseType as PyClassBaseType>::LayoutAsBase,
            contents: MaybeUninit<PyCellContents<T>>,
        }

        let (init, super_init) = match self.0 {
            PyClassInitializerImpl::Existing(value) => return Ok(value.into_ptr()),
            PyClassInitializerImpl::New { init, super_init } => (init, super_init),
        };

        let obj = super_init.into_new_object(py, subtype)?;

        let cell: *mut PartiallyInitializedPyCell<T> = obj as _;
        std::ptr::write(
            (*cell).contents.as_mut_ptr(),
            PyCellContents {
                value: ManuallyDrop::new(UnsafeCell::new(init)),
                borrow_checker: <T::PyClassMutability as PyClassMutability>::Storage::new(),
                thread_checker: T::ThreadChecker::new(),
                dict: T::Dict::INIT,
                weakref: T::WeakRef::INIT,
            },
        );
        Ok(obj)
    }

    private_impl! {}
}
// https://github.com/PyO3/pyo3/blob/54ab9090be944d72acbb78445c043b0c0e12a2ae/src/pyclass_init.rs#L258