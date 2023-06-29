pub fn from_box(boxed: Box<Wtf8>) -> Wtf8Buf {
    let bytes: Box<[u8]> = unsafe { mem::transmute(boxed) };
    Wtf8Buf { bytes: bytes.into_vec() }
}

/*
https://github.com/rusty-horizon/nx-rs/blob/d9861e0cf93028c26f6bf08238b8a0b0ecdd3f4a/nx-std/src/libstd/sys_common/wtf8.rs#L364
*/