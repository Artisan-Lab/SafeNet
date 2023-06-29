fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let ptr = data.as_ptr();
    let len = data.len();
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let new_slice = &slice[1..4];
    println!("{:?}", new_slice);
}