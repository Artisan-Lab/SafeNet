use std::rc::Rc;

fn main() {
    let v = vec![1,2,3];
    let mut rc = Rc::new(v);
    (*Rc::get_mut(&mut rc).unwrap())[1] = 24;
    assert_eq!((*rc)[1], 24);
}
