fn main() {
    let my_speed: Box<i32> = Box::new(88);
    let my_speed: *mut i32 = Box::into_raw(my_speed);
    unsafe {
        drop(Box::from_raw(my_speed));
    }
}