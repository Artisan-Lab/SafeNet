use std::mem::MaybeUninit;
fn main(){
let mut data: [MaybeUninit<String>; 1000] = unsafe { MaybeUninit::uninit().assume_init() };
// 初始化了500个String变量
let mut data_len: usize = 0;
for elem in &mut data[0..500] {
    //write没有将所有权转移出ManuallyDrop
    elem.write(String::from("hello"));
    data_len += 1;
}
}

/*
https://github.com/Artisan-Lab/Rust-SGNN/blob/main/data/assume_init/8-array-maybeuninitstring-unsafe.rs
*/