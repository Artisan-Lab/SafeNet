
fn main() {
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];
    let num = u32::from_le_bytes(raw_bytes);
    //let num = u32::from_ne_bytes(raw_bytes);
    //let num = u32::from_be_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);
}
