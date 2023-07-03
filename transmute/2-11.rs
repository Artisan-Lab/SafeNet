pub fn from_box(boxed: Box<Slice>) -> Buf {
    let inner: Box<str> = unsafe { mem::transmute(boxed) };
    Buf { inner: inner.into_string() }
    let boxed: Box<str> = self.inner.into();
    unsafe { mem::transmute(boxed) }

}
/*
https://github.com/theseus-os/Theseus/blob/e6e255f2f8bff2d0fdd6e6ccba362c36b93ab5e0/ports/theseus_std/src/os_str_imp.rs#L136
*/