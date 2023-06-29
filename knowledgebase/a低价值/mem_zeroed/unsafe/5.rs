use std::mem::MaybeUninit;

enum NotZero { One = 1, Two = 2 }

let x = MaybeUninit::<(u8, NotZero)>::zeroed();
let x = unsafe { x.assume_init() };
// Inside a pair, we create a `NotZero` that does not have a valid discriminant.
// This is undefined behavior. ⚠️