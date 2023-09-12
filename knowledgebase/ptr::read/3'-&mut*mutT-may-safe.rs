fn main() {
    let mut x: i32 = 42;
    let reference: &mut i32 = &mut x; // 使用可变引用
    println!("Read value: {}", *reference);
}
