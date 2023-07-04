#![allow(unused)]

fn main() {
    fn deref(&self) -> &'static *mut PyObject {
        &self.inner.TimeZone_UTC
    }

}