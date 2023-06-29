fn main() {
    let mut nums = [1, 2, 3, 4, 5];
    let target = 4;

    unsafe {
        let ptr = nums.as_mut_ptr();
        let len = nums.len();
        let mut i = 0;

        while i < len {
            let val = *ptr.offset(i as isize);
            if val == target {
                println!("Target found at index {}", i);
                break;
            }
            i += 1;
        }

        if i == len {
            println!("Target not found");
        }
    }
}
