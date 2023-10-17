use std::ptr;

fn main() {
    let mut outer_vec: Vec<Vec<i32>> = Vec::new();

    // Initialize the outer vector
    outer_vec.push(vec![1, 2, 3]);
    outer_vec.push(vec![4, 5, 6]);

    // Set the length of the outer vector to 0
    unsafe { outer_vec.set_len(0); }

    // Create another array to copy the inner vectors
    let mut copied_array: Vec<Vec<i32>> = Vec::new();

    // Copy the inner vectors to the new array using ptr::copy
    copy_vec(&outer_vec as *const _, 2, &mut copied_array);

    // Now you can use the copied array of inner vectors
    for inner_vec in copied_array {
        println!("Copied Vec: {:?}", inner_vec);
    }
}

fn copy_vec<'a>(p: *const Vec<Vec<i32>>, len: u32, des: &'a mut Vec<Vec<i32>>) { }
