#![allow(unused)]
fn main() {
    let ptr: *const u8 = &20u8 as *const u8;

    unsafe {
        let val_back = &*ptr;
        println!(val_back);
    }
}