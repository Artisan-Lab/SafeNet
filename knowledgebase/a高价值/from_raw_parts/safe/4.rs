fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let mut slice = &mut data[..];
    let new_slice = std::mem::replace(&mut slice, &mut []);
    let replaced_slice = &new_slice[1..4];
    println!("{:?}", replaced_slice);
}