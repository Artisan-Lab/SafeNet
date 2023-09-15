use std::ptr;

fn main() {
    let mut uninitialized_data: i32;

    let uninitialized_data_ptr: *mut i32 = &mut uninitialized_data;

    unsafe {
        ptr::write_bytes(uninitialized_data_ptr, 0, 1); // 试图写入一个字节到未初始化的内存
    }

    println!("uninitialized_data: {}", uninitialized_data);
}
