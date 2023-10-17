/*
use std::mem::MaybeUninit;

fn foo<T>() -> MaybeUninit<T> {
    MaybeUninit::uninit()
}
*/
fn main() {
    let value: MaybeUninit<u32> = foo();
    let initialized_value = unsafe { value.assume_init() };
    // println!("Initialized value: {}", initialized_value);
}