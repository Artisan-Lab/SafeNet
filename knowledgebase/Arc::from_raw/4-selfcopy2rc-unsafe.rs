//use std::sync::Arc;

//struct MyStruct {v: i32}


impl MyStruct {
    fn to_arc(&mut self) -> Arc<i32>{
        let p = &mut self.v as *mut i32;
        unsafe{ Arc::from_raw(p) }
    }
}

/*
fn main() {
    let mut x = MyStruct { v:1 };
    x.to_arc();
}
*/