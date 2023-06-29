fn main() {
    let x = &[1, 2, 4];
    let ptr = x.as_ptr();
    let second_elem_ref = unsafe { &*ptr.add(1) };
    assert_eq!(second_elem_ref, &2);
}
