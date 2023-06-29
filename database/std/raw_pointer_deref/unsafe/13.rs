fn main() {
    let array: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 获取长度为 5 的子数组的裸指针
    let ptr = &array[0] as *const u32;
    let ptr = ptr as *const [u32; 5];

    // 使用裸指针和 unsafe 代码块获取长度为 5 的子数组
    let result: &[u32; 5] = unsafe { &*ptr };
    
    println!("{:?}", result);
}
