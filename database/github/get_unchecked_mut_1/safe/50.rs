pub fn encode_varint32(mut value: u32, buf: &mut [u8; 5]) -> usize {
    let mut i = 0;
    while (value & !0x7F) > 0 {
        buf[i] = ((value & 0x7F) | 0x80) as u8;
        value >>= 7;
        i += 1;
    }
    buf[i] = value as u8;
    i + 1
}