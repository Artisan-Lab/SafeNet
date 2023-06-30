#![allow(unused)]
fn main() {
    let ptr: *const u8 = &10u8 as *const u8;

    unsafe {
        let val_back = &*ptr;
        println!("We got back the value: {val_back}!");
    }
}