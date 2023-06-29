#![allow(unused)]
use std::ptr;

fn main() {

    fn swap<T>(a: &mut T, b: &mut T) {
        unsafe {
            let tmp = ptr::read(a);
            ptr::copy_nonoverlapping(b, a, 1);
            ptr::write(b, tmp);

        }
    }

    let mut foo = "foo".to_owned();
    let mut bar = "bar".to_owned();

    swap(&mut foo, &mut bar);
}