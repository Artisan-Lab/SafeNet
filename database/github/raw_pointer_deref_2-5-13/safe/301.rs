fn foo(slice: &[u32]) -> &[u32] {
    if slice.len() == 5 {
        &slice[0..5]
    } else {
        panic!("Needs to be length 5")
    }
}