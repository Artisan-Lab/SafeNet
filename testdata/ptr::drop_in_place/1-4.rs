#[allow(unused_unsafe)]
            extern "C" fn dispose_helper(src: *mut std::os::raw::c_void) {
                unsafe {
                    std::ptr::drop_in_place(src as *mut BlockLiteral);
                }
            }

// https://github.com/makepad/makepad/blob/17eef68bd1347eca1b920390642f9f44f25e4a6f/libs/objc-sys/src/lib.rs#L56