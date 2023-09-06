////use std::rc::Rc;

//struct MyStruct {v: i32}


// impl MyStruct {
    fn to_rc(&mut self) -> Rc<i32>{
        let p = &mut self.v as *mut i32;
        unsafe{ Rc::from_raw(p) }
    }
// }

/*
fn main() {
    let mut x = MyStruct { v:1 };
    x.to_rc();
}
*/