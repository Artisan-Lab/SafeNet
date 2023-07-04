use std::rc::Rc;

struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> *mut Data {
        let data = Box::new(Data { value });
        Box::into_raw(data)
    }
}

fn main() {

    let raw_ptr = Data::new(42);
    let rc = unsafe { Rc::from_raw(raw_ptr) };
    println!("Value: {}", rc.value);
}
