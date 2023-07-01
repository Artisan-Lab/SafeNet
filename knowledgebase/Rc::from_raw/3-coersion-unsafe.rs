/*
  Repaceable: no
*/

//#![allow(unused)]
//use std::rc::Rc;
/*
fn main() {
    let s = "123";
    foo(s);
}
*/
pub fn foo(s: &str) -> Rc<Vec<u8>> {
    let rc: Rc<&str> = Rc::from(s);
    unsafe { 
        Rc::from_raw(Rc::into_raw(rc) as *const Vec<u8>) 
    }
}
