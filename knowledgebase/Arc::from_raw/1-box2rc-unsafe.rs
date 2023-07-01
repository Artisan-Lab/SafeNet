/*
    Add a new case...
    Replaceable: Yes
*/

//use std::sync::Arc;

/*
struct MyStruct {
    value: i32,
}
*/
fn main() {
    let b = Box::new( MyStruct { value: 42 });
    let p: *mut MyStruct = Box::into_raw(b);
    unsafe { (*p).value = 24; }
    let rc = unsafe { Arc::from_raw(p) };
    //println!("Value: {}", rc.value); // Output: Value: 24
}
