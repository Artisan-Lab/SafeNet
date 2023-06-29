fn parse_sbyte<'a>(data: &'a [u8], offset: usize, count: usize)
                   -> Value<'a> {
    let uslice = &data[offset .. offset + count];
    let islice = unsafe { ::std::slice::from_raw_parts(
        uslice.as_ptr() as *const i8, count) };
    Value::SByte(islice.to_vec())
}
