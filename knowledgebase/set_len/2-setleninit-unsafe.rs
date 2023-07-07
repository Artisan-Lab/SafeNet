// fn foo(vec: &mut Vec<i32>) {
//     // Some initialization logic
//     for i in 0..vec.len() {
//         vec[i] = i as i32 + 1;
//     }
// }

fn main() {
    let mut my_vec: Vec<i32> = Vec::new();

    // Reserve capacity for 10 elements
    my_vec.reserve(10);

    // Set the desired length of the vector
    unsafe { my_vec.set_len(10); }

    // Call a function to initialize the vector
    foo(&mut my_vec);

    // Now you can use the initialized vector
    println!("Initialized Vec: {:?}", my_vec);
}