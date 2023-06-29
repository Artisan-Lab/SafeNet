use std::num::NonZeroU16;

let x = unsafe { NonZeroU16::new_unchecked(42) };
println!("{}", x.get());
