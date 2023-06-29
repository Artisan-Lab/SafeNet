let x = MaybeUninit::<(u8, bool)>::new((0, false));
let x = x.assume_init();
assert_eq!(x, (0, false));
