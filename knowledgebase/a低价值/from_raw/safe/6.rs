use std::rc::Rc;

fn main() {
    let x = Rc::new(42);
    // let x_clone = x.clone();
    // let x = Rc::clone(&x_clone);
    assert_eq!(*x, 42);
}
