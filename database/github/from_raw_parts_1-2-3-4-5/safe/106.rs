fn parse_sbyte<'a>(data: &'a [u8], offset: usize, count: usize)
                   -> Value<'a> {
    let bytes = data[offset .. offset + count].into_iter()
        .map(|x| *x as i8)
        .collect();
    Value::SByte(bytes)
}