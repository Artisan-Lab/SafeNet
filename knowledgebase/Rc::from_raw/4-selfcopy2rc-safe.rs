//use std::rc::Rc;

struct MyStruct {v:i32}

impl MyStruct {
    fn to_rc(&mut self) -> Rc<i32>{
        Rc::new(self.v)
    }
}
fn main() {
    let mut x = MyStruct { v:1 };
    x.to_rc();
}