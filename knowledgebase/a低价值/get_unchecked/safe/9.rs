fn main() {
    let mut arr = [0; 1000000];
    let count = arr.iter().filter(|&x| *x > 100).count();
    println!("There are {} elements greater than 100", count);
}