extern "C" fn resource_destructor<T>(_env: NIF_ENV, handle: MUTABLE_NIF_RESOURCE_HANDLE) {
    unsafe {
        let aligned = align_alloced_mem_for_struct::<T>(handle);
        let res = aligned as *mut T;
        ptr::read(res);
    }
}