/*
    Replaceable: no
*/

fn increment(x: usize) {
    let p = x as *mut u32;
    unsafe { Rc::increment_strong_count(p) };
}

/*
fn main() {
    let rc = Rc::new(42);
    // Convert the Rc to raw pointer
    let p = Rc::into_raw(rc) as usize;
    // Pass the raw pointer to the function
    increment(p);
    let rc = unsafe { Rc::<u32>::from_raw(p as *const _) };
    println!("{}",Rc::strong_count(&rc));
}
*/