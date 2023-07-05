/*
https://github.com/Drecc/chromium-mojo/blob/09070518ef53a8aeb89babc78f0da944cae23277/mojo/public/rust/system/message_pipe.rs#L179
*/
pub fn read(
        &self,
        _flags: ReadFlags,
    ) -> Result<(vec::Vec<u8>, vec::Vec<handle::UntypedHandle>), MojoResult> {
        // Read the message, yielding a message object we can copy data from.
        let message_handle = {
            let mut h = 0;
            let result = MojoResult::from_code(unsafe {
                ffi::MojoReadMessage(self.handle.get_native_handle(), ptr::null(), &mut h as *mut _)
            });
            if result != MojoResult::Okay {
                return Err(result);
            }
            h
        };

        let mut buffer: *mut c_void = ptr::null_mut();
        let mut num_bytes: u32 = 0;
        let mut num_handles: u32 = 0;
        let result_prelim = MojoResult::from_code(unsafe {
            ffi::MojoGetMessageData(
                message_handle,
                ptr::null(),
                &mut buffer as *mut _,
                &mut num_bytes as *mut _,
                ptr::null_mut(),
                &mut num_handles as *mut _,
            )
        });
        if result_prelim != MojoResult::Okay && result_prelim != MojoResult::ResourceExhausted {
            return Err(result_prelim);
        }

        let mut raw_handles: vec::Vec<MojoHandle> = vec::Vec::with_capacity(num_handles as usize);
        if num_handles > 0 {
            let raw_handles_ptr = raw_handles.as_mut_ptr();
            let result = MojoResult::from_code(unsafe {
                ffi::MojoGetMessageData(
                    message_handle,
                    ptr::null(),
                    &mut buffer as *mut _,
                    &mut num_bytes as *mut _,
                    raw_handles_ptr,
                    &mut num_handles as *mut _,
                )
            });
            if result != MojoResult::Okay {
                return Err(result);
            }
        }

        let data: Vec<u8> = if num_bytes > 0 {
            assert_ne!(buffer, ptr::null_mut());
            // Will not panic if usize has at least 32 bits, which is true for our targets
            let buffer_size: usize = num_bytes.try_into().unwrap();
            // MojoGetMessageData points us to the data with a c_void pointer and a length. This
            // is only available until we destroy the message. We want to copy this into our own
            // Vec. Read the buffer as a slice, which is safe.
            unsafe {
                let buffer_slice = std::slice::from_raw_parts(buffer.cast(), buffer_size);
                buffer_slice.to_vec()
            }
        } else {
            Vec::new()
        };

        unsafe {
            raw_handles.set_len(num_handles as usize);
        }
        let mut handles: vec::Vec<handle::UntypedHandle> =
            vec::Vec::with_capacity(num_handles as usize);
        for raw_handle in raw_handles.iter() {
            handles.push(unsafe { handle::acquire(*raw_handle) });
        }

        unsafe {
            ffi::MojoDestroyMessage(message_handle);
        }

        Ok((data, handles))
    }