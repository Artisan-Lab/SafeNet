#![allow(unused)]
fn main() {
    use std::alloc::{dealloc, Layout};
    use std::ptr;

    let x = Box::new(String::from("Hello"));
    let p = Box::into_raw(x);
    unsafe {
        ptr::drop_in_place(p);
        dealloc(p as *mut u8, Layout::new::<String>());
        // 由于 String 类型并没有显式实现 Drop trait，因此需要手动调用 std::alloc::dealloc() 函数来释放指向 Box<String> 的内存，同时也需要传递适当的布局信息 Layout::new::<String>()，以确保正确的内存大小和对齐方式
    }
}



// 同delloc 1
// std::ptr::drop_in_place() 函数可以用于释放指向实现了 Drop trait 的数据类型的指针
// 数据类型包括 Rust 内建的基本类型，以及标准库和用户自定义的复杂数据类型，例如 String、Vec<T>、HashMap<K, V>、Rc<T>、Arc<T> 等

// 此代码 1 释放string 
// 扩充：2 释放基本类型 i32   
// 扩充：19 释放自定义类型Vec<u8>
// 扩充： 20 释放包含Vec容器中的对象

//本质上修改都是drop


/*
Rust 中的基本类型17：
布尔类型（bool）
整数类型（i8、i16、i32、i64、i128、isize 和无符号整数类型 u8、u16、u32、u64、u128、usize）
浮点数类型（f32、f64）
字符类型（char）
字节类型（u8）

Rust 的基本类型（如整数、浮点数、布尔值等）并不需要实现 Drop trait，因为它们的生命周期和所有权都是由编译器自动管理的，它们的内存分配和释放都是在编译时处理的。因此，基本类型的实例可以在离开作用域时自动被释放，不需要手动调用 drop() 方法来释放它们的资源
对于包含基本类型的结构体或枚举等自定义类型，如果它们包含需要释放的资源（如堆上分配的内存），则需要实现 Drop trait 来释放这些资源
*/

// 在 Rust 中，一般情况下不需要手动释放容器。一些特殊情况如使用 Box::into_raw 将一个对象转换为原始指针后，需要手动释放内存
        