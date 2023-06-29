fn main(){
    let mut vec = vec![1, 2, 3];
    // 使用 vec.truncate(2) 将 vec 截断为长度为 2。这意味着第三个元素将从 vec 中移除
    vec.truncate(2);
    assert_eq!(vec, vec![1, 2]);
}
