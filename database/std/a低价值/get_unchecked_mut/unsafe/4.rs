fn main() {
    let mut vec = vec![1, 2, 3];
    // 创建了一个 Vec 对象，并使用 get_unchecked_mut 方法获取一个元素切片
    let slice = unsafe { vec.get_unchecked_mut(1..3) };
    for elem in slice {
        *elem += 1;
    }
    println!("{:?}", vec); 
}