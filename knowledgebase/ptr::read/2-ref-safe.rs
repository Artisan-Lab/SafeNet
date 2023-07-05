#![allow(unused)]
use std::ptr;

fn foo(s:&mut String) {
    let s2 = "bar";
    *s = "bar".to_string();
}
fn main()
{
    let mut a = "1".to_string();
    foo(&mut a);
    println!("{}",a);
    
}