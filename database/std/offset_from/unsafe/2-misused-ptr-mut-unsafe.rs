#![allow(unused)]
fn main() {
    let ptr1 = Box::into_raw(Box::new(0u8));
    let ptr2 = Box::into_raw(Box::new(1u8));
    let diff = (ptr2 as isize).wrapping_sub(ptr1 as isize);
    let ptr2_other = (ptr1 as *mut u8).wrapping_offset(diff);

    unsafe {
        let zero = ptr2_other.offset_from(ptr2); // Undefined Behavior
    }
}