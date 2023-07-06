use std::mem::MaybeUninit;

/*
fn init_value(value: &mut MaybeUninit<i32>) {
    unsafe {
        value.as_mut_ptr().write(42);
    }
}
*/

fn main() {
    let mut uninitialized_value = MaybeUninit::<i32>::uninit();
    init_value(&mut uninitialized_value);
    let initialized_value = unsafe { uninitialized_value.assume_init() };
    // println!("Initialized value: {}", initialized_value);
}
