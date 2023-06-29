fn main() {
    let x: u32 = 0x1234_5678;
    let y: [u8; 4] = unsafe { std::intrinsics::transmute(x) };
    println!("{:X} = {:X} {:X} {:X} {:X}", x, y[0], y[1], y[2], y[3]);
}
