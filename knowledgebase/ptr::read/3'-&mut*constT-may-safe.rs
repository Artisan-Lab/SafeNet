fn main() {
    let mut x: i32 = 42;
    let reference: &mut i32 = &mut x; 
    *reference = 10;
    println!("Modified value: {}", *reference);
}
