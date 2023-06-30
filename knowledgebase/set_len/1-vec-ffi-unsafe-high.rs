/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.set_len
    Purpose: create a buffer of vec for FFI usage
    Replaceable? No
*/

#![allow(unused)]
#![allow(dead_code)]

pub struct StreamWrapper { strm: *mut std::ffi::c_void }
 const Z_OK: i32 = 0;

impl StreamWrapper {
    pub fn get_dictionary(&self) -> Option<Vec<u8>> {
        let mut dict = Vec::with_capacity(32_768);
        let mut dict_length = 0;
        unsafe {
            let r = deflateGetDictionary(self.strm, dict.as_mut_ptr(), &mut dict_length);
            if r == Z_OK {
                dict.set_len(dict_length);
                Some(dict)
            } else {
                None
            }
        }
    }
}
                                 
extern "C" {
    fn deflateGetDictionary(
        strm: *mut std::ffi::c_void,
        dictionary: *mut u8,
        dictLength: *mut usize,
    ) -> i32;
}

fn main() {
   
}
