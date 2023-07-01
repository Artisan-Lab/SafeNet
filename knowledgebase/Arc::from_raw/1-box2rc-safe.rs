use std::sync::Arc;

struct MyStruct {
    value: i32,
}

fn main() {

    let mut rc = Rc::new( Box::new(MyStruct { value: 42 }) );
    (*Arc::get_mut(&mut rc).unwrap()).value = 24;
    assert_eq!((*rc).value, 24);
}
