fn main() {
    let x = 42;
    let reference: &i32 = &x;
    let value: i32 = *reference;
    println!("Value: {}", value);
}
