use std::mem::MaybeUninit;

let x = MaybeUninit::<(u8, bool)>::zeroed();
let x = unsafe { x.assume_init() };
assert_eq!(x, (0, false));