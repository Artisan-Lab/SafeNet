fn main() {
    let mut v: Vec<Option<(i32, i32)>> = vec![None, Some((1, 2)), Some((3, 4))];
    let idx = 1;
    let n = unsafe { v.get_unchecked_mut(idx) };
    let (k, v) = n.take().unwrap();
    
}

