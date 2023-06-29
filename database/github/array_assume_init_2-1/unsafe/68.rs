#[inline]
fn decode(decoder: &mut Decoder<'a>) -> Result<IntEncodedWithFixedSize, String> {
    let mut bytes = MaybeUninit::uninit_array();
    let _start_pos = decoder.position();
    decoder.read_raw_bytes(&mut bytes)?;

    let _end_pos = decoder.position();
    debug_assert_eq!((_end_pos - _start_pos), IntEncodedWithFixedSize::ENCODED_SIZE);

    let value = u64::from_le_bytes(unsafe { MaybeUninit::array_assume_init(bytes) });

    Ok(IntEncodedWithFixedSize(value))
}