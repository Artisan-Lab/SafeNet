fn main() {
    let x: u32 = 0x1234_5678;
    let y: [u8; 4] = x.to_le_bytes();
    println!("{:X} = {:X} {:X} {:X} {:X}", x, y[0], y[1], y[2], y[3]);
}
