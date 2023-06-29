unsafe fn get_first_element(ptr: *const Vec<i32>) -> i32 {
    match ptr.as_ref() {
        Some(v) => *v.get(0).unwrap(),
        None => panic!("Null pointer!"),
    }
}
