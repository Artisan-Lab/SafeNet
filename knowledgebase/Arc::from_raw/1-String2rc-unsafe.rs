/*
    From:
    Replaceable: Yes
*/

//use std::sync::Arc;

fn main() {
    let mut s = String::from("123");
    s.push('s');
    s.push_str("123");
    let rc = Arc::new(s);
    println!("{}", *rc);
}