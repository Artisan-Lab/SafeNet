#![allow(unused)]
fn main() {
    use std::slice;

// manifest a slice for a single element
    let mut x = 42;
    let mut ptr =&mut x as &mut i32;
    let slice = unsafe { slice::from_raw_parts_mut(ptr, 1) };
    // safe方式，使用from_mut替换from_raw_parts_mut，因为此代码创建的切片只包含一个元素

    assert_eq!(slice[0], 42);

}