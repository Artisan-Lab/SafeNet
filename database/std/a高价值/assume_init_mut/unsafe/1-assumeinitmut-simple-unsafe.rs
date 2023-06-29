#![allow(unused)]
#![allow(unexpected_cfgs)]
use std::mem::MaybeUninit;

unsafe extern "C" fn initialize_buffer(buf: *mut [u8; 1024]) { *buf = [0; 1024] }

#[cfg(FALSE)]
extern "C" {
    /// Initializes *all* the bytes of the input buffer.
    fn initialize_buffer(buf: *mut [u8; 1024]);
}

fn main() {
    let mut buf = MaybeUninit::<[u8; 1024]>::uninit();
    unsafe { initialize_buffer(buf.as_mut_ptr()); }
    let buf: &mut [u8; 1024] = unsafe {
        // SAFETY: `buf` has been initialized.
        buf.assume_init_mut()
    };
}