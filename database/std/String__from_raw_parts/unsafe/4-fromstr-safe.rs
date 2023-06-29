use std::mem;

fn foo(s1: &mut str) {
    let s = s1.to_string();
    assert_eq!(s, "hello");
}

fn main() {
    let s = String::from("hello");
    let mut s = mem::ManuallyDrop::new(s);
    let s1 = s.as_mut_str();
    foo(s1);
}
