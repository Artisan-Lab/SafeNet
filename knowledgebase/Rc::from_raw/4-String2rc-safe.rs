use std::rc::Rc;

fn main() {
    let mut s = String::from("123");
    s.push('s');
    s.push_str("123");
    let rc = Rc::new(s);
    println!("{}", *rc);
}
