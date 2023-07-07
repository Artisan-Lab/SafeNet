// fn foo(vec: &mut Vec<i32>) {
//     // Some initialization logic
//     for i in 1..=10 {
//         vec.push(i);
//     }
// }

fn main() {
    let mut my_vec: Vec<i32> = Vec::new();
    // Reserve capacity for 10 elements
    my_vec.reserve(10);
    // Call a function to initialize the vector
    foo(&mut my_vec);
    // Set the length of the vector based on the initialized elements
    let len = my_vec.len();
    unsafe { my_vec.set_len(len); }
    // Now you can use the initialized and properly sized vector
    println!("Initialized Vec: {:?}", my_vec);
}