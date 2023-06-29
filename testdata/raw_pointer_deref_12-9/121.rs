#![allow(unused)]

fn main() {
    fn deref(&self) -> &crate::PyObject {
        unsafe {
            &*((&self.inner.TimeZone_UTC) as *const *mut crate::ffi::PyObject
                as *const crate::PyObject)
        }
    }

}