#![allow(unused)]
fn main() {
    let ptr: *const u8 = &10u8 as *const u8;

    unsafe {
        if let Some(val_back) = ptr.as_ref() {
            println!("We got back the value: {val_back}!");
        }
    }
}