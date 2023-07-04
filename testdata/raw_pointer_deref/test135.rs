fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let ptr = nums.as_ptr();
    unsafe {
        for i in 0..5 {
            let num_ptr = ptr.add(i);
            if *num_ptr > 3 {
                break;
            }
            println!("Element {}: {}", i, *num_ptr);
        }
    }
}
