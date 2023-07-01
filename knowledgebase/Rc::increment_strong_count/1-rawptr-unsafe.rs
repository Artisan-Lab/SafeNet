/*
    Replaceable: no
*/

//use std::rc::Rc;

fn increment(p: *const u32) {
    unsafe { Rc::increment_strong_count(p) };
}
/*
fn main() {
    let rc = Rc::new(42);
    // Convert the Rc to raw pointer
    let p = Rc::into_raw(rc);
    // Pass the raw pointer to the function
    increment(p);
    let rc = unsafe { Rc::<u32>::from_raw(p) };
    println!("{}",Rc::strong_count(&rc));
}
*/