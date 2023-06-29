fn main() {
    let v = vec![1, 2, 3];
    let mut v = std::mem::ManuallyDrop::new(v);
    let len = v.len();
    let mut v_new = Vec::with_capacity(len);

    for i in 0..len {
        v_new.push(4 + i);
    }
    std::mem::drop(v);
    assert_eq!(v_new, [4, 5, 6]);
}
