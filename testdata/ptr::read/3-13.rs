fn shift_tail<T, F>(v: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    // SAFETY: The unsafe operations below involves indexing without a bound check (by offsetting a
    // pointer) and copying memory (`ptr::copy_nonoverlapping`).
    //
    // a. Indexing:
    //  1. We checked the size of the array to >= 2.
    //  2. All the indexing that we will do is always between `0 <= index < len-1` at most.
    //
    // b. Memory copying
    //  1. We are obtaining pointers to references which are guaranteed to be valid.
    //  2. They cannot overlap because we obtain pointers to difference indices of the slice.
    //     Namely, `i` and `i+1`.
    //  3. If the slice is properly aligned, the elements are properly aligned.
    //     It is the caller's responsibility to make sure the slice is properly aligned.
    //
    // See comments below for further detail.
    unsafe {
        // If the last two elements are out-of-order...
        if len >= 2 && is_less(v.get_unchecked(len - 1), v.get_unchecked(len - 2)) {
            // Read the last element into a stack-allocated variable. If a following comparison
            // operation panics, `hole` will get dropped and automatically write the element back
            // into the slice.
            let tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(len - 1)));
            let v = v.as_mut_ptr();
            let mut hole = CopyOnDrop::new(&*tmp, v.add(len - 2));
            ptr::copy_nonoverlapping(v.add(len - 2), v.add(len - 1), 1);

            for i in (0..len - 2).rev() {
                if !is_less(&*tmp, &*v.add(i)) {
                    break;
                }

                // Move `i`-th element one place to the right, thus shifting the hole to the left.
                ptr::copy_nonoverlapping(v.add(i), v.add(i + 1), 1);
                hole.dest = v.add(i);
            }
            // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        }
    }
}
// https://github.com/rayon-rs/rayon/blob/f47f4ff9e1e06d3088204a44e686a6ea5317a8a0/src/slice/quicksort.rs#L73