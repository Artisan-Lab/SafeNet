/*
    From:
    Replaceable: Yes
*/

//use std::rc::Rc;

fn main() {
    let mut s = String::from("123");
    s.push('s');
    s.push_str("123");
    let p = Box::into_raw(Box::new(s)); 
    let rc = unsafe { Rc::from_raw(p) };
    println!("{}", *rc);
}
