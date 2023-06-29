fn get_mut(acc: &mut [u8], idx: usize) -> &mut u8 {
    unsafe { acc.get_unchecked_mut(idx) }
}
/*
https://github.com/facebook/fbthrift/blob/334bc616557da6ed703cd0b30fc02ecc2dafb24a/thrift/lib/rust/src/varint.rs#L37
*/