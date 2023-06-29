fn collect_into_array<I, const N: usize>(iter: &mut I) -> Option<[I::Item; N]>
where
    I: Iterator,
{
    if N == 0 {
        // SAFETY: An empty array is always inhabited and has no validity invariants.
        return unsafe { Some(mem::zeroed()) };
    }

    struct Guard<T, const N: usize> {
        ptr: *mut T,
        initialized: usize,
    }

    impl<T, const N: usize> Drop for Guard<T, N> {
        fn drop(&mut self) {
            debug_assert!(self.initialized <= N);

            let initialized_part = crate::ptr::slice_from_raw_parts_mut(self.ptr, self.initialized);

            // SAFETY: this raw slice will contain only initialized objects.
            unsafe {
                crate::ptr::drop_in_place(initialized_part);
            }
        }
    }

    let mut array = MaybeUninit::uninit_array::<N>();
    let mut guard: Guard<_, N> =
        Guard { ptr: MaybeUninit::slice_as_mut_ptr(&mut array), initialized: 0 };

    while let Some(item) = iter.next() {
        // SAFETY: `guard.initialized` starts at 0, is increased by one in the
        // loop and the loop is aborted once it reaches N (which is
        // `array.len()`).
        unsafe {
            array.get_unchecked_mut(guard.initialized).write(item);
        }
        guard.initialized += 1;

        // Check if the whole array was initialized.
        if guard.initialized == N {
            mem::forget(guard);

            // SAFETY: the condition above asserts that all elements are
            // initialized.
            let out = unsafe { MaybeUninit::array_assume_init(array) };
            return Some(out);
        }
    }

    // This is only reached if the iterator is exhausted before
    // `guard.initialized` reaches `N`. Also note that `guard` is dropped here,
    // dropping all already initialized elements.
    None
}