#![allow(unused)]
fn main() {
    let ptr: *mut u8 = &mut 10u8 as *mut u8;

    unsafe {
        if let Some(val_back) = ptr.as_ref() {
            println!("We got back the value: {val_back}!");
        }
    }
}

// 使用NonNull::new()来创建一个NonNull实例，然后通过map()方法将裸指针转换为Option<u8>类型，并使用as_ptr()来获取裸指针的值
// use std::ptr::NonNull;
// fn main(){
//     let ptr: *mut u8 = &mut 10u8 as *mut u8;
//     let non_null = NonNull::new(ptr);
//     let value: Option<u8> = non_null.map(|nn| unsafe { *nn.as_ptr() });
//     if let Some(val_back) = value.as_ref() {
//         println!("We got back the value: {}!", val_back);
//     }
// }