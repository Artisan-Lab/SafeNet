/*
  Repaceable: yes.Create Arc<B> from A => Convert A to B first 
*/

//#![allow(unused)]
//use std::sync::Arc;
/*
fn main() {
    let s = "123";
    foo(s);
}
*/
pub fn foo(s: &str) -> Arc<Vec<u8>> {
    let rc: Arc<&str> = Arc::from(s);
    unsafe { 
        Rc::from_raw(Arc::into_raw(rc) as *const Vec<u8>) 
    }
}
