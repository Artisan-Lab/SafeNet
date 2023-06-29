fn main() {
    use std::alloc::{alloc, Layout};

    // 分配一个大小为 i32 的空间，返回一个原始指针（*mut i32）。
    let ptr = alloc(Layout::new::<i32>()) as *mut i32;
    // 用 5 写入这个指针指向的内存空间。
    unsafe {
        ptr.write(5);
    }
    
    // 把原始指针封装到一个 Box 内部指针，使之变成一个安全指针，并返回 Box。
    let x = unsafe { Box::from_raw(ptr) };
    // 确认 x 内部指向的值是 5。
    assert_eq!(*x,5)
}
