#![allow(unused)]
fn main() {
    let ptr: *const u8 = &10u8 as *const u8;

    unsafe {
        // 如果指针不为null，则通过as_ref()方法将指针转化为Option<&T>类型，并获取其中的值
        if let Some(val_back) = ptr.as_ref() {
            println!("We got back the value: {}!", val_back);
        }
    }
}

/*
as_ref()函数将指针转换为一个引用类型，但是我们并不能保证该指针指向的内存地址确实存储了一个有效的值。
这段代码使用了指针，并在 unsafe 块中使用了 as_ref() 方法将其转换为了 Option<&T> 类型，并尝试获取其值。
不安全原因：使用指针可能会导致悬垂指针、内存泄漏、非法内存访问等问题。在本例中，使用了指向整型字面量 10 的指针，而该字面量在栈上分配内存，可能被回收。另外，指针也可能指向未初始化的内存。
 */
fn main() {
    let ptr: *const u8 = &10u8 as *const u8;
    unsafe {
        if let Some(val_back) = ptr.as_ref() {
            println!("We got back the value: {}!", val_back);
        }
    }
}
    