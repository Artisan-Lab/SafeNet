fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; 
    let element = slice.get(1); // 使用 get 方法替代 get_unchecked 方法
    if let Some(e) = element {
        println!("{}", e);
    } else {
        println!("Failed");
    }
}