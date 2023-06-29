#[inline]
fn decode(decoder: &mut Decoder<'a>) -> Result<IntEncodedWithFixedSize, String> {

    let _start_pos = decoder.position();

    let bytes = decoder.read_raw_bytes(IntEncodedWithFixedSize::ENCODED_SIZE);
    let _end_pos = decoder.position();
    debug_assert_eq!((_end_pos - _start_pos), IntEncodedWithFixedSize::ENCODED_SIZE);


    let value = u64::from_le_bytes(bytes.try_into().unwrap());
    Ok(IntEncodedWithFixedSize(value))
}