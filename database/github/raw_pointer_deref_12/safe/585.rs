unsafe fn find_value(ptr: *const i32, len: usize, target: i32) -> Option<usize> {
    let mut i = 0;
    loop {
        if i >= len {
            break None;
        }
        if *ptr.offset(i as isize) == target {
            break Some(i);
        }
        i += 1;
    }
}
