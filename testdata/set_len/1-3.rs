/*
https://github.com/otcshare/chromium-src/blob/3b920d87437d9293f654de1f22d3ea341e7a8b55/mojo/public/rust/system/data_pipe.rs#L292
*/

pub fn read(&self, flags: ReadFlags) -> Result<Vec<T>, MojoResult> {
        let mut options = ffi::MojoReadDataOptions::new(ReadFlags::QUERY.bits());
        let mut num_bytes: u32 = 0;
        let r_prelim = unsafe {
            ffi::MojoReadData(
                self.handle.get_native_handle(),
                options.inner_ptr(),
                ptr::null_mut() as *mut ffi::c_void,
                &mut num_bytes as *mut u32,
            )
        };
        if r_prelim != 0 || num_bytes == 0 {
            return Err(MojoResult::from_code(r_prelim));
        }

        options.flags = flags.bits();
        let elem_size: u32 = mem::size_of::<T>() as u32;
        // TODO(mknyszek): make sure elem_size divides into num_bytes
        let mut buf: vec::Vec<T> = vec::Vec::with_capacity((num_bytes / elem_size) as usize);
        let r = MojoResult::from_code(unsafe {
            ffi::MojoReadData(
                self.handle.get_native_handle(),
                options.inner_ptr(),
                buf.as_mut_ptr() as *mut ffi::c_void,
                &mut num_bytes as *mut u32,
            )
        });
        unsafe { buf.set_len((num_bytes / elem_size) as usize) }
        if r != MojoResult::Okay { Err(r) } else { Ok(buf) }
    }