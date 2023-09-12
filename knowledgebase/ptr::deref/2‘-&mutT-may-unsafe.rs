fn main() {
    let mut x = 42;
    let raw_ptr: *mut i32 = &mut x;
    let reference: &i32 = unsafe { &*raw_ptr }; // 可以在没有其他可变引用的情况下安全替换
    println!("Value: {}", reference);
}