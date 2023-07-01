#![allow(unused)]
use std::sync::Arc;


fn main() {
    let s = "123";
    foo(s);
}

pub fn foo(s: &str) -> Arc<Vec<u8>> {
    let v = s.as_bytes().to_vec();
    Arc::from(v)
}
