fn default() -> Self {
    let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
    unsafe {
        ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
        s.assume_init()
    }
}
// https://github.com/firecracker-microvm/firecracker/blob/a9300c13c823e4192a042a2d0c765e0bae5a6319/src/net_gen/src/iff.rs#L940