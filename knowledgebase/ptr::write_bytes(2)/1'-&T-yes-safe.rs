fn main() {
    let x: i32 = 42;
    let reference: &i32 = &x; 
    println!("Read value: {}", *reference);
}
