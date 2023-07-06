fn default() -> Self {
    let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
    unsafe {
        ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
        s.assume_init()
    }
}