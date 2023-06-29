#![allow(unused)]
// fn main() {
//     let array = [0, 1, 2];
//     assert_eq!(array, [0, 1, 2]);
// }


fn main() {
    let mut array: [i32; 3] = [0; 3];
    // 对数组中的元素进行写入操作
    array[0] = 0;
    array[1] = 1;
    array[2] = 2;

    assert_eq!(array, [0, 1, 2]);
}
