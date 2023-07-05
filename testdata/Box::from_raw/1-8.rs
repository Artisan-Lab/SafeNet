extern "C" fn exception_cleanup(
    _unwind_code: uw::_Unwind_Reason_Code,
    exception: *mut uw::_Unwind_Exception,
) {
    unsafe {
        let _: Box<Exception> = Box::from_raw(exception as *mut Exception);
        super::__rust_drop_panic();
    }
}

// https://github.com/bjorn3/rust/blob/2f896da247e0ee8f0bef7cd7c54cfbea255b9f68/library/panic_unwind/src/gcc.rs#L79