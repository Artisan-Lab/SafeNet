#![allow(unused)]
#![feature(allocator_api, new_uninit)]

fn main() {
    let values = Box::<[u32]>::try_new_zeroed_slice(3).unwrap();
    let values = unsafe { values.assume_init() };

    assert_eq!(*values, [0, 0, 0]);

}

//Box::try_new_zeroed_slice() 函数尝试创建一个堆分配的数组类型 Box，并将其所有字节设置为零。但是，如果内存分配失败或者数组类型不正确，则会返回一个错误值。在第一段代码中，我们使用了 try_new_zeroed_slice() 函数，并在返回的结果上使用了 unwrap() 方法，这意味着如果创建失败，程序将会崩溃
// 与10的区别而 Box::new_zeroed_slice() 函数创建一个堆分配的数组类型 Box，并将其所有字节设置为零。与 try_new_zeroed_slice() 不同，它不会返回错误值，因为它不尝试在创建时检查内存分配失败或者类型错误