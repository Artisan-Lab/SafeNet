fn main() {
    let mut vec = vec![1, 2, 3];
    unsafe {
       vec.set_len(2);
       }
       assert_eq!(vec, vec![1, 2]);
}