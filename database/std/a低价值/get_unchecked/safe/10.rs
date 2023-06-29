fn main() {
    let mut arr = [0; 1000000];

    for (i, val) in arr.iter_mut().enumerate() {
        if i % 2 == 0 {
            *val = i as i32;
        }
    }

    let count = arr.iter().filter(|&x| *x > 100).count();
    println!("There are {} elements greater than 100", count);
}