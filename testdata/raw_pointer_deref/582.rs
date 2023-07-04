unsafe fn sum_array(arr: *const i32, len: usize) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < len {
        let val = *arr.offset(i as isize);
        sum += val;
        i += 1;
    }
    sum
}
