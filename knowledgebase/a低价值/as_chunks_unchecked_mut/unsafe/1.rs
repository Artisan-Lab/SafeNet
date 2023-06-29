#![allow(unused)]
#![feature(slice_as_chunks)]
fn main() {
    // 
    let slice: &mut [char] = &mut ['l', 'o', 'r', 'e', 'm', '!'];

    // 将 slice 转化为一组由 [char; 1] 组成的 slice，这里用到了 unsafe
    let chunks: &mut [[char; 1]] =
        unsafe { slice.as_chunks_unchecked_mut() };

    // 修改第一个 chunk 的值为 ['L']
    chunks[0] = ['L'];

    // 断言 chunks 数组的值为 [['L'], ['o'], ['r'], ['e'], ['m'], ['!']]
    assert_eq!(chunks, &[['L'], ['o'], ['r'], ['e'], ['m'], ['!']]);

    // 将 slice 转化为一组由 [char; 3] 组成的 slice，这里用到了 unsafe
    let chunks: &mut [[char; 3]] =
        unsafe { slice.as_chunks_unchecked_mut() };

    // 修改第二个 chunk 的值为 ['a', 'x', '?']
    chunks[1] = ['a', 'x', '?'];

    // 断言 slice 的值为 ['L', 'o', 'r', 'a', 'x', '?']
    assert_eq!(slice, &['L', 'o', 'r', 'a', 'x', '?']);
}
