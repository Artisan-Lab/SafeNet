pub unsafe fn get_disjoint_unchecked_mut<const N: usize>(
    &mut self,
    keys: [K; N],
) -> [&mut V; N] {
    // Safe, see get_disjoint_mut.
    let mut ptrs: [MaybeUninit<*mut V>; N] = MaybeUninit::uninit().assume_init();
    for i in 0..N {
        ptrs[i] = MaybeUninit::new(self.get_unchecked_mut(keys[i]));
    }
    core::mem::transmute_copy::<_, [&mut V; N]>(&ptrs)
}

// https://github.com/GaiaWorld/pi_lib/blob/27bcd61b78caebf2f3665df63e48db76df07f727/slotmap/src/delay.rs#L753