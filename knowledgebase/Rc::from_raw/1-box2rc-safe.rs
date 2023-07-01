use std::rc::Rc;

struct MyStruct {
    value: i32,
}

fn main() {

    let mut rc = Rc::new( MyStruct { value: 42 });
    (*Rc::get_mut(&mut rc).unwrap()).value = 24;
    assert_eq!((*rc).value, 24);
}
