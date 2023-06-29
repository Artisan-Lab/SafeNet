#![allow(unused)]
#![feature(ptr_as_uninit)]

fn main() {
    let ptr: *const u8 = &10u8 as *const u8;

    unsafe {
        // 将指针转化为Option<MaybeUninit<&T>>类型，并获取其中的值
        if let Some(val_back) = ptr.as_uninit_ref() {
            // 对MaybeUninit<&T>进行assume_init()方法调用，将其转换为&T类型的引用
            println!("We got back the value: {}!", val_back.assume_init());
        }
    }
}

