//#![allow(unused)]
//use std::sync::Arc;
//fn main() {
    let x = Arc::new("hello".to_owned());
    let x_ptr = Arc::into_raw(x);

    unsafe {
        let x = Arc::from_raw(x_ptr);

    }
//}
