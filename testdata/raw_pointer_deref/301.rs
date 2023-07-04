fn foo(array: &[u32; 10]) -> &[u32; 5] {
    let slice = &array[0..5];

    if slice.len() == 5 {
        let ptr = slice.as_ptr() as *const [u32; 5];
        unsafe { &*ptr }
    } else {
        panic!("Needs to be length 5")
    }
}
