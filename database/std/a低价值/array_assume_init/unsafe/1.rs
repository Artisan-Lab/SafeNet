#![allow(unused)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
fn main() {
    use std::mem::MaybeUninit;

    let mut array: [MaybeUninit<i32>; 3] = MaybeUninit::uninit_array();
    // 对数组中的元素进行写入操作
    array[0].write(0);
    array[1].write(1);
    array[2].write(2);

    let array = unsafe {
    // 使用unsafe块将可能未初始化的数组转化为已初始化的数组
        MaybeUninit::array_assume_init(array)
    };

    assert_eq!(array, [0, 1, 2]);

}


