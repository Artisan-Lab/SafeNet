//use std::rc::Rc;

fn main() {
    let v = vec![1,2,3];
    let p = Box::into_raw(Box::new(v));
    unsafe { (*p)[1] = 24; }
    let rc = unsafe { Rc::from_raw(p) };
    assert_eq!((*rc)[1], 24);
}
