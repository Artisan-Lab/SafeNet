////use std::rc::Rc;

//struct MyStruct {v: Vec<u8>}

impl MyStruct {
    fn to_rc(&mut self) -> Rc<Vec<u8>>{
        let p = &mut (self.v) as *mut Vec<u8>;
        unsafe {Rc::from_raw(p)}
    }
}
/*
fn main() {
    let mut x = MyStruct { v:vec![1,2,3] };
    x.to_rc();
}
*/