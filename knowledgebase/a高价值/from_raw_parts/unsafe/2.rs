fn main() {
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr(); // 获取数组的指针
    let len = arr.len(); // 获取数组的长度
    unsafe {
        let slice = std::slice::from_raw_parts(ptr, len); // 使用from_raw_parts方法将指针转换为切片
        println!("{:?}", slice); 
    }
}
