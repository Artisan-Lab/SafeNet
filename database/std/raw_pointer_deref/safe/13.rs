fn main() {
    let array: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 使用 slice 获取长度为 5 的子数组
    let slice = &array[0..5];

    // 将 slice 转换为一个 &[u32; 5] 类型的引用
    let result: &[u32; 5] = match slice.try_into() {
        Ok(reference) => reference,
        Err(_) => panic!("Needs to be length 5")
    };

    println!("{:?}", result);
}
