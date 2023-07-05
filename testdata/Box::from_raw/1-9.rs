unsafe extern "C" fn destroy_value<T: 'static>(ptr: *mut u8) {
    if let Err(_) = panic::catch_unwind(|| unsafe {
        let ptr = Box::from_raw(ptr as *mut Value<T>);
        let key = ptr.key;
        key.os.set(ptr::invalid_mut(1));
        drop(ptr);
        key.os.set(ptr::null_mut());
    }) {
        rtabort!("thread local panicked on drop");
    }
}
}

// https://github.com/esp-rs/rust/blob/ed3726ba7aea45731260ec8f75f05fc60c2b0f22/library/std/src/sys/common/thread_local/os_local.rs#L188