fn main() {
    let mut data = [0u8; 10];
    let mut index = 5;

    data[index] = 42;

    assert_eq!(data, [0, 0, 0, 0, 0, 42, 0, 0, 0, 0]);
}
