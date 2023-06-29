fn main() {
    // 创建一个二维向量，表示单位矩阵
    let mut vec = vec![vec![1, 0, 0],
                       vec![0, 1, 0],
                       vec![0, 0, 1]];
    // 使用 `set_len` 方法将向量长度设置为 0，因此向量中不再包含任何元素
    unsafe {
        vec.set_len(0);
    }
    // 断言向量为空，即长度为 0
    assert!(vec.is_empty());
}
