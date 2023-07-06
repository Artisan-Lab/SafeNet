fn main() {
    let store = [0, 1, 2, 3];
    let v = store.iter().collect::<Vec<&i32>>();
    let v = unsafe {
        let v_ptr: *const Vec<&i32> = &v;
        let v_copy: Vec<Option<&i32>> = std::mem::transmute_copy(&v_ptr);
        v_copy
    };
}
