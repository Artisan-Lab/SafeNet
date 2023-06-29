#![feature(slice_ptr_get)]
fn main(){
    let x = &[1, 2, 4] as *const [i32];
    unsafe {
        assert_eq!(x.get_unchecked(1), x.as_ptr().add(1));
}
}