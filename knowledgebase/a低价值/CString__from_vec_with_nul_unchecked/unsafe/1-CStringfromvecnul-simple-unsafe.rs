#![allow(unused)]
use std::ffi::CString;

fn main() {
    assert_eq!(
        unsafe { CString::from_vec_with_nul_unchecked(b"abc\0".to_vec()) },
        unsafe { CString::from_vec_unchecked(b"abc".to_vec()) }
    );
}