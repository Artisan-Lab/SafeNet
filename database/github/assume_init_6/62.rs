use std::mem::MaybeUninit;
fn initialize_vector() -> Vec<i32> {
    let v: MaybeUninit<Vec<i32>> = MaybeUninit::uninit();
    let mut vec_box = unsafe { v.assume_init() };
    vec_box.push(1);
    vec_box.push(2);
    vec_box.push(3);

}