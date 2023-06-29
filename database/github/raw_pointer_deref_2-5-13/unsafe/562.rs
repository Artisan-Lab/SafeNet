fn main() {
    let mut nums = [1, 2, 3, 4, 5];

    unsafe {
        let ptr = nums.as_mut_ptr();
        let len = nums.len();
        let mut i = 0;

        while i < len {
            let val = *ptr.offset(i as isize);
            if val == 3 {
                continue;
            }
            println!("Current value is {}", val);
            i += 1;
        }
    }
}