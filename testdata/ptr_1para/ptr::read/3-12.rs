fn shift_head<T, F>(v: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    // SAFETY: The unsafe operations below involves indexing without a bounds check (by offsetting a
    // pointer) and copying memory (`ptr::copy_nonoverlapping`).
    //
    // a. Indexing:
    //  1. We checked the size o[](../../../../../../rayon-rs/rayon/blob/f47f4ff9e1e06d3088204a44e686a6ea5317a8a0/src/slice/quicksort.rs)f the array to >=2.
    //  2. All the indexing that we will do is always between {0 <= index < len} at most.
    //
    // b. Memory copying
    //  1. We are obtaining pointers to references which are guaranteed to be valid.
    //  2. They cannot overlap because we obtain pointers to difference indices of the slice.
    //     Namely, `i` and `i-1`.
    //  3. If the slice is properly aligned, the elements are properly aligned.
    //     It is the caller's responsibility to make sure the slice is properly aligned.
    //
    // See comments below for further detail.
    unsafe {
        // If the first two elements are out-of-order...
        if len >= 2 && is_less(v.get_unchecked(1), v.get_unchecked(0)) {
            // Read the first element into a stack-allocated variable. If a following comparison
            // operation panics, `hole` will get dropped and automatically write the element back
            // into the slice.
            let tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(0)));
            let v = v.as_mut_ptr();
            let mut hole = CopyOnDrop::new(&*tmp, v.add(1));
            ptr::copy_nonoverlapping(v.add(1), v.add(0), 1);

            for i in 2..len {
                if !is_less(&*v.add(i), &*tmp) {
                    break;
                }

                // Move `i`-th element one place to the left, thus shifting the hole to the right.
                ptr::copy_nonoverlapping(v.add(i), v.add(i - 1), 1);
                hole.dest = v.add(i);
            }
            // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        }
    }
}

// https://github.com/rayon-rs/rayon/blob/f47f4ff9e1e06d3088204a44e686a6ea5317a8a0/src/slice/quicksort.rs#L73
