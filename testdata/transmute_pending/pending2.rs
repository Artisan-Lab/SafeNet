pub fn from_box(boxed: Box<Slice>) -> Buf {
    let inner: Box<Wtf8> = unsafe { mem::transmute(boxed) };
    Buf { inner: Wtf8Buf::from_box(inner) }
}
/*
https://github.com/alyssaverkade/rust/blob/cb390735b03aa44229ff2858be8fedbd7b0ce7cb/library/std/src/sys/windows/os_str.rs#L137
*/