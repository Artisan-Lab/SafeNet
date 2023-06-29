#![allow(unused)]
#![feature(slice_as_chunks)]
fn main() {
    // 创建一个字符切片
    let slice: &[char] = &['l', 'o', 'r', 'e', 'm', '!'];
    // 使用 as_chunks_unchecked 方法将字符切片分成大小为 1 的块，并返回一个由大小为 1 的数组组成的数组的引用
    let chunks: &[[char; 1]] =
        unsafe { slice.as_chunks_unchecked() };
    assert_eq!(chunks, &[['l'], ['o'], ['r'], ['e'], ['m'], ['!']]);
    // 使用 as_chunks_unchecked 方法将字符切片分成大小为 3 的块，并返回一个由大小为 3 的数组组成的数组的引用
    let chunks: &[[char; 3]] =
        unsafe { slice.as_chunks_unchecked() };
    assert_eq!(chunks, &[['l', 'o', 'r'], ['e', 'm', '!']]);
}

/*
这段代码中使用了 as_chunks_unchecked 方法来将一个字符切片分成多个固定大小的块，并返回一个数组的引用，数组中的元素是由原字符切片中的元素组成的固定大小的数组
as_chunks_unchecked 不安全因为它假定了字符 slice 的内部表示方式，并且没有进行任何边界检查
*/
