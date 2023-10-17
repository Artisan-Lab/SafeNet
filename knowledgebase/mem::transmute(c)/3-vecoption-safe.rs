#![allow(unused)]
fn main() {
    let store = [0, 1, 2, 3];
    let v_orig = store.iter().collect::<Vec<&i32>>();
    let v_clone = v_orig.clone();
    let v_collected = v_clone.into_iter()
                         .map(Some)
                         .collect::<Vec<Option<&i32>>>();
}
