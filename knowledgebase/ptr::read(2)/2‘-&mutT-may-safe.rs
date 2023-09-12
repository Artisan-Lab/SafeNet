fn main() {
    let mut x: i32 = 42;
    let reference: &mut i32 = &mut x; 
    println!("Read value: {}", *reference);
}
