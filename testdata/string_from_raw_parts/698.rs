/*
    From: https://github.com/yetone/leveldb-rs/blob/0c07a9b6d23353b2dba1d1406396d39733f35d9e/src/memtable.rs#L194
*/

fn get_string_from_slice(slice: &Slice) -> String {
    unsafe {
        let mut v: Vec<u8> = Vec::with_capacity(slice.size());
        bit::memcpy(&mut v, slice.data());
        String::from_raw_parts(v.as_mut_ptr(), slice.size(), slice.size())
    }
}