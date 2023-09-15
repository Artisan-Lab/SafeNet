fn main() {
    let mut values = vec![1, 2, 3];

    values.iter_mut().for_each(|x| *x = 0);

    println!("{:?}", values);
}
