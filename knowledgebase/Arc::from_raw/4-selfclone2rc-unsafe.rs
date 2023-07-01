//use std::sync::Arc;

//struct MyStruct {v: Vec<u8>}

impl MyStruct {
    fn to_arc(&mut self) -> Arc<Vec<u8>>{
        let p = &mut (self.v) as *mut Vec<u8>;
        unsafe {Arc::from_raw(p)}
    }
}
/*
fn main() {
    let mut x = MyStruct { v:vec![1,2,3] };
    x.to_arc();
}
*/