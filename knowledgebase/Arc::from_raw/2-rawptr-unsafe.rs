/*
  From: 
  Replaceable: No
*/

//#![allow(unused)]
//use std::rc::Rc;

fn foo(p: *mut u32){
    unsafe { Rc::from_raw(p); }
}

/*
fn main() {
    let x = Box::new(5);
    let p = Box::into_raw(x);
    foo(p);
}
*/
