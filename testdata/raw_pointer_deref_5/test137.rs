fn main() {
    let nums = [1, 2, 3, 4, 5];
    let ptr = nums.as_ptr();
    unsafe {
        for i in 0..nums.len() {
            let val = *ptr.offset(i as isize);
            if val % 2 == 0 {
                println!("Even number: {}", val);
            }
        }
    }
}
