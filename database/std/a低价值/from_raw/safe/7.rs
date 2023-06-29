use std::rc::Rc;

fn main() {
    let rc = Rc::new(42);
    assert_eq!(*rc, 42);
}
