use std::ffi::{CString, CStr};
use std::os::raw::c_char;

fn function(x: Vec<u8>) -> CString {
    let y = x.into_boxed_slice();
    let x = y.into_vec();
    unsafe { CString::from_raw(x.as_ptr() as *mut c_char) }
}

fn main() {
    let x = vec![72, 101, 108, 108, 111]; // "Hello"
    let z = function(x);

    // 打印 Vec 中的值
    println!("{:?}", z); // [72, 101, 108, 108, 111]
}
