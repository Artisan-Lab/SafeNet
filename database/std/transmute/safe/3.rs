use std::convert::TryFrom;

fn main() {
    let x: u32 = 12345;
    let y = i32::try_from(x).unwrap();
    println!("{}", y);
}
