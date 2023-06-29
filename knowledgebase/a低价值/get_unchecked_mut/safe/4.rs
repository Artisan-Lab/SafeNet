fn main() {
    let mut vec = vec![1, 2, 3];
    if let Some(slice) = vec.get_mut(1..3) {
        for elem in slice {
            *elem += 1;
        }
        println!("{:?}", vec); 
    }
}