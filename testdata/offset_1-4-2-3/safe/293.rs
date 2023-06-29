fn main() {
    let mut buffer = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa];

    let a: u32 = 0xdeadc0de;
    let a_bytes = a.to_le_bytes();
    buffer[0..4].copy_from_slice(&a_bytes);

    let b: u16 = 0xbadf;
    let b_bytes = b.to_le_bytes();
    buffer[4..6].copy_from_slice(&b_bytes);

    println!("{:x?}", buffer);
}
