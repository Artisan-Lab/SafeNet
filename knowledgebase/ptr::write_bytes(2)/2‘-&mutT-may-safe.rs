fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    for elem in arr.iter_mut() {
        *elem = 0;
    }

    println!("Array after zeroing: {:?}", arr);
}
