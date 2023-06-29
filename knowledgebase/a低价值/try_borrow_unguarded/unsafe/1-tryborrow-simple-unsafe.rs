#![allow(unused)]
use std::cell::RefCell;

fn main() {
    let c = RefCell::new(5);

    {
        let m = c.borrow_mut();
        assert!(unsafe { c.try_borrow_unguarded() }.is_err());
    }

    {
        let m = c.borrow();
        assert!(unsafe { c.try_borrow_unguarded() }.is_ok());
    }
}