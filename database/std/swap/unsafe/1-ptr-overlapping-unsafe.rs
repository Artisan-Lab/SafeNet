#![allow(unused)]
fn main() {
    use std::ptr;

    let mut array: [i32; 4] = [0, 1, 2, 3];

    let array_ptr: *mut i32 = array.as_mut_ptr();

    let x = array_ptr as *mut [i32; 3]; // this is `array[0..3]`
    let y = unsafe { array_ptr.add(1) } as *mut [i32; 3]; // this is `array[1..4]`

    unsafe {
        ptr::swap(x, y);
    }
}