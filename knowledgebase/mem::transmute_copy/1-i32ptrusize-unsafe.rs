fn main() {
    let ptr = &0;
    let ptr_num = unsafe {
        std::mem::transmute_copy::<&i32, usize>(&ptr)
    };
}
