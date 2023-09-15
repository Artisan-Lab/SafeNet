use std::ptr;

fn main() {
    let mut num: i32 = 0;
    let float_data = 3.14159265359f32;

    unsafe {
        let num_ptr: *mut i32 = &mut num;
        let float_data_ptr: *const u8 = &float_data as *const f32 as *const u8;
        ptr::write_bytes(num_ptr, *float_data_ptr, 4); // 试图写入 4 个字节的浮点数字节表示到整数内存中
    }

    println!("num: {}", num);
}
