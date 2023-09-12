fn main() {
    let mut x = 42;
    let reference: &mut i32 = &mut x;
    let value: i32 = *reference;
    println!("Value: {}", value);
}
