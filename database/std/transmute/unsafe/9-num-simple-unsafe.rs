fn main() {
    let x: u32 = 12345;
    let y: i32 = unsafe { std::mem::transmute(std::num::NonZeroU32::new(x)) };
    println!("{} as i32 is {}", x, y);
}
