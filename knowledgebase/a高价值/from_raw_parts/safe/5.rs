use std::slice;
fn join_slices<'a, T>(fst: &'a [T], snd: &'a [T]) -> Vec<T>
where
    T: Copy,
{
    assert_eq!(fst.len(), snd.len(), "Slices must have the same length!");
    let mut result = vec![fst[0]; fst.len() + snd.len()];
    result[..fst.len()].copy_from_slice(fst);
    result[fst.len()..].copy_from_slice(snd);
    result
}

fn main() {
    let a = 42;
    let b = 27;
    let _ = join_slices(slice::from_ref(&a), slice::from_ref(&b));
}
