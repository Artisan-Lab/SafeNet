// use std::rc::Rc;
// use std::ptr;

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    let rc_data = Rc::new(data);
    let raw_ptr = Rc::into_raw(rc_data.clone());
    let mutable_ptr = raw_ptr as *mut Vec<i32>;
    unsafe {
        let data_mut = &mut *mutable_ptr;
        for i in 0..data_mut.len() {
            *data_mut.get_mut(i).unwrap() += 1;
        }
    }
    let modified_rc_data = unsafe { Rc::from_raw(raw_ptr) };

    // println!("{:?}", modified_rc_data); 

}
