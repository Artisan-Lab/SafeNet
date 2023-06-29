fn main() {
    let mut arr = [0; 1000000];
    let mut count = 0;

    for i in 0..arr.len() {
        let val = unsafe { arr.get_unchecked_mut(i) };
        if *val > 100 {
            count += 1;
        }
    }

    println!("There are {} elements greater than 100", count);
}
