pub fn from_box(boxed: Box<Wtf8>) -> Wtf8Buf {
    let bytes: Box<[u8]> = unsafe { mem::transmute(boxed) };
    Wtf8Buf { bytes: bytes.into_vec() }
}
/*
https://github.com/pombredanne/rust-sgx-sdk-1/blob/aede10425d8baec18746d523bd6c97f183675802/sgx_tstd/src/sys_common/wtf8.rs#L381
*/ 