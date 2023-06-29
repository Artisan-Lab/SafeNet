#![allow(unused)]
fn main() {
    use std::alloc::{dealloc, Layout};
    use std::ptr;

    // 分配一个堆上的字符串
    let x = Box::new(String::from("Hello"));
    // 把 Box 转换成裸指针
    let p = Box::into_raw(x);

    unsafe {
        // 手动调用指针指向对象的析构函数
        ptr::drop_in_place(p);

        // 使用正确的布局释放内存
        dealloc(p as *mut u8, Layout::new::<String>());
    }
}

// dealloc 用于释放任何类型的内存，drop_in_place 只能用于实现了 Drop trait 的类型
// dealloc 需要传递一个额外的 Layout 对象，以便确定内存块的大小和对齐方式，而 drop_in_place 不需要
/* 
首先使用 ptr::drop_in_place 函数手动调用指针指向对象的析构函数，以确保在释放内存之前对象的资源得到正确的清理。
然后使用 dealloc 函数释放指针指向的内存，并使用 Layout::new::<String>() 函数来确定要释放的内存的正确布局
*/

/*调用了 ptr::drop_in_place 函数销毁 String 实例，然后使用 dealloc 函数将分配的内存释放回去 */

//1、2的主要区别是分配方式和释放函数不同