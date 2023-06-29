use std::mem;

fn main() {
    let ptr: *mut i32 = unsafe { mem::zeroed() };
    println!("ptr is {:?}", ptr);
}