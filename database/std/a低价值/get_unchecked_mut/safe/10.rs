fn main() {
    let mut v: Vec<Option<(i32, i32)>> = vec![None, Some((1, 2)), Some((3, 4))];
    let idx = 1;
    if let Some(n) = v.get_mut(idx).take() {
        let (k, v) = n;
    }
}