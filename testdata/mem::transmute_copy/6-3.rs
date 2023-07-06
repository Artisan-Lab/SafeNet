fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> [T; N] {
    let mut buff: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    for elem in &mut buff {
        *elem = MaybeUninit::new(_rng.gen());
    }

    unsafe { mem::transmute_copy::<_, _>(&buff) }
}