use std::mem::MaybeUninit;

fn main() {
    let x = unsafe{MaybeUninit::<i32>::zeroed().assume_init();}
    println!("Initialized value: {}", x);
}

