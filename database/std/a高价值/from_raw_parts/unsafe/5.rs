use std::slice;

fn join_slices<'a, T>(fst: &'a [T], snd: &'a [T]) -> &'a [T] {
    let fst_end = fst.as_ptr().wrapping_add(fst.len());
    let snd_start = snd.as_ptr();
    assert_eq!(fst_end, snd_start, "Slices must be contiguous!");
    unsafe {
        slice::from_raw_parts(fst.as_ptr(), fst.len() + snd.len())
    }
}

fn main() {
    let a = 42;
    let b = 27;
    let _ = join_slices(slice::from_ref(&a), slice::from_ref(&b)); // UB
} 