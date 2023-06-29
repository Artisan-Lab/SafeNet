fn main() {
    let mut arr: [i32; 1] = [0; 1];
    for elem in arr.iter_mut() {
        *elem = 42;
    }
    let p = arr.as_mut_ptr();
    let v = *p;
}
