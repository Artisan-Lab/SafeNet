impl<T: Init, const LEN: usize> Init for [T; LEN] {
    const INIT: Self = {
        // `<[T; LEN]>::from_fn` is not `const fn` [ref:const_array_from_fn]
        let mut array = mem::MaybeUninit::uninit_array();

        // `[T]::iter` is unusable in `const fn` [ref:const_slice_iter]
        // FIXME: `needless_range_loop` false positive
        // <https://github.com/rust-lang/rust-clippy/issues/10524>
        #[expect(clippy::needless_range_loop)]
        for i in 0..LEN {
            array[i] = mem::MaybeUninit::new(T::INIT);
        }

        // Safety: `array`'s elements are fully initialized
        unsafe { mem::MaybeUninit::array_assume_init(array) }
    };
}
/*
https://github.com/r3-os/r3/blob/e7696aa1d66aa74a935b31bb894e1f274bcc4206/src/r3_core/src/utils/init.rs#L50
*/
