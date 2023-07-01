use std::sync::Arc;

struct MyStruct {v:i32}

impl MyStruct {
    fn to_arc(&mut self) -> Arc<i32>{
        Arc::new(self.v)
    }
}
fn main() {
    let mut x = MyStruct { v:1 };
    x.to_arc();
}