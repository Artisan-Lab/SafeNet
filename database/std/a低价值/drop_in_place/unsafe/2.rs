fn main() {
    let mut x = 123;
    let p = &mut x as *mut i32;
    // 通过 &mut x 取出 x 的可变引用，并将其转换成了一个 *mut i32 类型的原始指针 p，表示指向 x 所在的内存地址

    unsafe {
        std::ptr::drop_in_place(p);
        // 手动释放了 i32 类型对象所占用的内存
    }
}



//// x 是基本类型，不需要实现 Drop trait
//// 整数是基本类型，其内存管理是由 Rust 编译器自动处理的（&&&基本类型是在栈上分配内存的，它们通常不需要手动释放&&&）

